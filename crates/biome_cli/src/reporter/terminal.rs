use crate::Reporter;
use crate::execute::{Execution, TraversalMode};
use crate::reporter::{
    DiagnosticsPayload, EvaluatedPathsDiagnostic, FixedPathsDiagnostic, ReporterVisitor,
    TraversalSummary,
};
use biome_console::fmt::Formatter;
use biome_console::{Console, ConsoleExt, fmt, markup};
use biome_diagnostics::PrintDiagnostic;
use biome_diagnostics::advice::ListAdvice;
use biome_fs::BiomePath;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeSet;
use std::io;
use std::time::Duration;

pub(crate) struct ConsoleReporter {
    pub(crate) summary: TraversalSummary,
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) evaluated_paths: BTreeSet<BiomePath>,
    pub(crate) working_directory: Option<Utf8PathBuf>,
    pub(crate) verbose: bool,
}

impl Reporter for ConsoleReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(
            &self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        if self.verbose {
            visitor
                .report_handled_paths(self.evaluated_paths, self.working_directory.as_deref())?;
        }
        visitor.report_summary(&self.execution, self.summary, self.verbose)?;
        Ok(())
    }
}

pub(crate) struct ConsoleReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for ConsoleReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        execution: &Execution,
        summary: TraversalSummary,
        verbose: bool,
    ) -> io::Result<()> {
        if execution.is_check() && summary.suggested_fixes_skipped > 0 {
            self.0.log(markup! {
                <Warn>"Skipped "{summary.suggested_fixes_skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --write --unsafe\n"</Emphasis></Info>
            })
        }

        if !execution.is_ci() && summary.diagnostics_not_printed > 0 {
            self.0.log(markup! {
                <Warn>"The number of diagnostics exceeds the limit allowed. Use "<Emphasis>"--max-diagnostics"</Emphasis>" to increase it.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{summary.diagnostics_not_printed}</Emphasis><Info>"."</Info>
            })
        }

        self.0.log(markup! {
            {ConsoleTraversalSummary(execution.traversal_mode(), &summary, verbose)}
        });

        Ok(())
    }

    fn report_handled_paths(
        &mut self,
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

        self.0.log(markup! {
            {PrintDiagnostic::verbose(&evaluated_paths_diagnostic)}
        });
        self.0.log(markup! {
            {PrintDiagnostic::verbose(&fixed_paths_diagnostic)}
        });

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        execution: &Execution,
        diagnostics_payload: DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if execution.is_search() {
                self.0.log(markup! {{PrintDiagnostic::search(diagnostic)}});
                continue;
            }

            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && verbose {
                    self.0
                        .error(markup! {{PrintDiagnostic::verbose(diagnostic)}});
                } else {
                    self.0
                        .error(markup! {{PrintDiagnostic::simple(diagnostic)}});
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

struct SummaryDetail<'a>(pub(crate) &'a TraversalMode, usize);

impl fmt::Display for SummaryDetail<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(mode, files) = self;
        if let TraversalMode::Search { .. } = mode {
            return Ok(());
        }

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

struct SummaryTotal<'a>(&'a TraversalMode, usize, &'a Duration);

impl fmt::Display for SummaryTotal<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(mode, files, duration) = self;
        let files = Files(*files);
        match mode {
            TraversalMode::Check { .. } | TraversalMode::Lint { .. } | TraversalMode::CI { .. } => {
                fmt.write_markup(markup! {
                    "Checked "{files}" in "{duration}"."
                })
            }
            TraversalMode::Format { write, .. } => {
                if *write {
                    fmt.write_markup(markup! {
                        "Formatted "{files}" in "{duration}"."
                    })
                } else {
                    fmt.write_markup(markup! {
                        "Checked "{files}" in "{duration}"."
                    })
                }
            }

            TraversalMode::Migrate { write, .. } => {
                if *write {
                    fmt.write_markup(markup! {
                      "Migrated your configuration file in "{duration}"."
                    })
                } else {
                    fmt.write_markup(markup! {
                        "Checked your configuration file in "{duration}"."
                    })
                }
            }

            TraversalMode::Search { .. } => fmt.write_markup(markup! {
                "Searched "{files}" in "{duration}"."
            }),
        }
    }
}

pub(crate) struct ConsoleTraversalSummary<'a>(
    pub(crate) &'a TraversalMode,
    pub(crate) &'a TraversalSummary,
    pub(crate) bool,
);
impl fmt::Display for ConsoleTraversalSummary<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let Self(mode, summary, verbose) = *self;
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
        let total = SummaryTotal(mode, summary.changed + summary.unchanged, &duration);
        let detail = SummaryDetail(mode, summary.changed);
        fmt.write_markup(markup!(<Info>{total}{detail}</Info>))?;

        // The search emits info diagnostics, so we use if control-flow to print a different message
        if let TraversalMode::Search { .. } = mode {
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
