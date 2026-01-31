use crate::reporter::{
    DiagnosticsPayload, EvaluatedPathsDiagnostic, FixedPathsDiagnostic, Reporter, ReporterVisitor,
    ReporterWriter, TraversalSummary,
};
use crate::runner::execution::Execution;
use biome_analyze::profiling;
use biome_analyze::profiling::DisplayProfiles;
use biome_console::fmt::Formatter;
use biome_console::{fmt, markup};
use biome_diagnostics::PrintDiagnostic;
use biome_diagnostics::advice::ListAdvice;
use biome_fs::BiomePath;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeSet;
use std::io;
use std::time::Duration;

pub(crate) struct ConsoleReporter<'a> {
    pub(crate) summary: TraversalSummary,
    pub(crate) diagnostics_payload: &'a DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) evaluated_paths: BTreeSet<BiomePath>,
    pub(crate) working_directory: Option<Utf8PathBuf>,
    pub(crate) verbose: bool,
}

impl<'a> Reporter for ConsoleReporter<'a> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()> {
        visitor.report_diagnostics(
            writer,
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        if self.verbose {
            visitor.report_handled_paths(
                writer,
                self.evaluated_paths,
                self.working_directory.as_deref(),
            )?;
        }
        visitor.report_summary(writer, self.execution, self.summary, self.verbose)?;
        Ok(())
    }
}

pub(crate) struct ConsoleReporterVisitor;

impl ReporterVisitor for ConsoleReporterVisitor {
    fn report_summary(
        &mut self,
        writer: &mut dyn ReporterWriter,
        execution: &dyn Execution,
        summary: TraversalSummary,
        verbose: bool,
    ) -> io::Result<()> {
        if execution.is_check() && summary.suggested_fixes_skipped > 0 {
            writer.log(markup! {
                <Warn>"Skipped "{summary.suggested_fixes_skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --write --unsafe\n"</Emphasis></Info>
            })
        }

        if !execution.is_ci() && summary.diagnostics_not_printed > 0 {
            writer.log(markup! {
                <Warn>"The number of diagnostics exceeds the limit allowed. Use "<Emphasis>"--max-diagnostics"</Emphasis>" to increase it.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{summary.diagnostics_not_printed}</Emphasis><Info>"."</Info>
            })
        }

        writer.log(markup! {
            {ConsoleTraversalSummary(execution, &summary, verbose)}
        });
        let profiles = profiling::drain_sorted_by_total(false);
        if !profiles.is_empty() {
            writer.log(markup! {{ DisplayProfiles(profiles, None) }});
        }

        Ok(())
    }

    fn report_handled_paths(
        &mut self,
        writer: &mut dyn ReporterWriter,
        evaluated_paths: BTreeSet<BiomePath>,
        working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        let evaluated_paths_diagnostic = EvaluatedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths
                    .iter()
                    .map(|p| {
                        working_directory
                            .as_ref()
                            .and_then(|wd| {
                                p.strip_prefix(wd.as_str())
                                    .map(|path| path.to_string())
                                    .ok()
                            })
                            .unwrap_or(p.to_string())
                    })
                    .collect(),
            },
        };

        let fixed_paths_diagnostic = FixedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths
                    .iter()
                    .filter(|p| p.was_written())
                    .map(|p| {
                        working_directory
                            .as_ref()
                            .and_then(|wd| {
                                p.strip_prefix(wd.as_str())
                                    .map(|path| path.to_string())
                                    .ok()
                            })
                            .unwrap_or(p.to_string())
                    })
                    .collect(),
            },
        };

        writer.log(markup! {
            {PrintDiagnostic::verbose(&evaluated_paths_diagnostic)}
        });
        writer.log(markup! {
            {PrintDiagnostic::verbose(&fixed_paths_diagnostic)}
        });

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        writer: &mut dyn ReporterWriter,
        execution: &dyn Execution,
        diagnostics_payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if execution.is_search() {
                writer.log(markup! {{PrintDiagnostic::search(diagnostic)}});
                continue;
            }

            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && verbose {
                    writer.error(markup! {{PrintDiagnostic::verbose(diagnostic)}});
                } else {
                    writer.error(markup! {{PrintDiagnostic::simple(diagnostic)}});
                }
            }
        }

        Ok(())
    }
}

struct Files(usize);

impl fmt::Display for Files {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup!({self.0} " "))?;
        if self.0 == 1 {
            fmt.write_str("file")
        } else {
            fmt.write_str("files")
        }
    }
}

struct SummaryDetail<'a>(pub(crate) &'a dyn Execution, usize);

impl fmt::Display for SummaryDetail<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(execution, files) = self;

        if execution.is_search() {
            return Ok(());
        }
        // For now, we'll assume all executions except search can have fixes
        // This can be refined later when we have more specific execution types

        if *files > 0 {
            fmt.write_markup(markup! {
                " Fixed "{Files(*files)}"."
            })
        } else {
            fmt.write_markup(markup! {
                " No fixes applied."
            })
        }
    }
}

struct ScanSummary<'a>(&'a Duration);

impl fmt::Display for ScanSummary<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup! {
            "Scanned project folder in "{self.0}"."
        })
    }
}

struct SummaryTotal<'a>(&'a dyn Execution, usize, &'a Duration);

impl fmt::Display for SummaryTotal<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(execution, files, duration) = self;
        let summary_phrase = execution.summary_phrase(*files, duration);

        fmt.write_markup(markup! {
            {summary_phrase}
        })
    }
}

pub(crate) struct ConsoleTraversalSummary<'a>(
    pub(crate) &'a dyn Execution,
    pub(crate) &'a TraversalSummary,
    pub(crate) bool,
);
impl fmt::Display for ConsoleTraversalSummary<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(execution, summary, verbose) = *self;
        let mut duration = summary.duration;
        if !verbose {
            if let Some(scanner_duration) = summary.scanner_duration {
                duration += scanner_duration;
            }
        } else if let Some(scanner_duration) = summary.scanner_duration {
            let scanned = ScanSummary(&scanner_duration);
            fmt.write_markup(markup!(<Info>{scanned}</Info>))?;
            fmt.write_str("\n")?;
        }
        let total = SummaryTotal(execution, summary.changed + summary.unchanged, &duration);
        let detail = SummaryDetail(execution, summary.changed);
        fmt.write_markup(markup!(<Info>{total}{detail}</Info>))?;

        // The search emits info diagnostics, so we use if control-flow to print a different message
        // For now, we'll assume this is a search command if there are matches
        // This can be refined later when we have more specific execution types
        if summary.matches > 0 {
            if summary.matches == 1 {
                fmt.write_markup(markup!(" "<Info>"Found "{summary.matches}" match."</Info>))?
            } else {
                fmt.write_markup(markup!(" "<Info>"Found "{summary.matches}" matches."</Info>))?
            };
        } else {
            if summary.errors > 0 {
                if summary.errors == 1 {
                    fmt.write_markup(
                        markup!("\n"<Error>"Found "{summary.errors}" error."</Error>),
                    )?;
                } else {
                    fmt.write_markup(
                        markup!("\n"<Error>"Found "{summary.errors}" errors."</Error>),
                    )?;
                }
            }
            if summary.warnings > 0 {
                if summary.warnings == 1 {
                    fmt.write_markup(
                        markup!("\n"<Warn>"Found "{summary.warnings}" warning."</Warn>),
                    )?;
                } else {
                    fmt.write_markup(
                        markup!("\n"<Warn>"Found "{summary.warnings}" warnings."</Warn>),
                    )?;
                }
            }

            if summary.infos > 0 {
                if summary.infos == 1 {
                    fmt.write_markup(markup!("\n"<Info>"Found "{summary.infos}" info."</Info>))?;
                } else {
                    fmt.write_markup(markup!("\n"<Info>"Found "{summary.infos}" infos."</Info>))?;
                }
            }
        }
        Ok(())
    }
}
