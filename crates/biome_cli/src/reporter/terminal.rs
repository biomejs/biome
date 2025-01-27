use crate::execute::{Execution, TraversalMode};
use crate::reporter::{DiagnosticsPayload, ReporterVisitor, TraversalSummary};
use crate::Reporter;
use biome_console::fmt::Formatter;
use biome_console::{fmt, markup, Console, ConsoleExt};
use biome_diagnostics::advice::ListAdvice;
use biome_diagnostics::{Diagnostic, PrintDiagnostic};
use biome_fs::BiomePath;
use std::collections::BTreeSet;
use std::io;
use std::time::Duration;

pub(crate) struct ConsoleReporter {
    pub(crate) summary: TraversalSummary,
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) evaluated_paths: BTreeSet<BiomePath>,
}

impl Reporter for ConsoleReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        let verbose = self.diagnostics_payload.verbose;
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload)?;
        visitor.report_summary(&self.execution, self.summary)?;
        if verbose {
            visitor.report_handled_paths(self.evaluated_paths)?;
        }
        Ok(())
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    tags(VERBOSE),
    severity = Information,
    message = "Files processed:"
)]
struct EvaluatedPathsDiagnostic {
    #[advice]
    advice: ListAdvice<String>,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    tags(VERBOSE),
    severity = Information,
    message = "Files fixed:"
)]
struct FixedPathsDiagnostic {
    #[advice]
    advice: ListAdvice<String>,
}

pub(crate) struct ConsoleReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for ConsoleReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        execution: &Execution,
        summary: TraversalSummary,
    ) -> io::Result<()> {
        if execution.is_check() && summary.suggested_fixes_skipped > 0 {
            self.0.log(markup! {
                <Warn>"Skipped "{summary.suggested_fixes_skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --fix --unsafe\n"</Emphasis></Info>
            })
        }

        if !execution.is_ci() && summary.diagnostics_not_printed > 0 {
            self.0.log(markup! {
                <Warn>"The number of diagnostics exceeds the limit allowed. Use "<Emphasis>"--max-diagnostics"</Emphasis>" to increase it.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{summary.diagnostics_not_printed}</Emphasis><Info>"."</Info>
            })
        }

        self.0.log(markup! {
            {ConsoleTraversalSummary(execution.traversal_mode(), &summary)}
        });

        Ok(())
    }

    fn report_handled_paths(&mut self, evaluated_paths: BTreeSet<BiomePath>) -> io::Result<()> {
        let evaluated_paths_diagnostic = EvaluatedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths.iter().map(|p| p.to_string()).collect(),
            },
        };

        let fixed_paths_diagnostic = FixedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths
                    .iter()
                    .filter(|p| p.was_written())
                    .map(|p| p.to_string())
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
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if execution.is_search() {
                self.0.log(markup! {{PrintDiagnostic::search(diagnostic)}});
                continue;
            }

            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && diagnostics_payload.verbose {
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
        if let TraversalMode::Search { .. } = self.0 {
            return Ok(());
        }

        if self.1 > 0 {
            fmt.write_markup(markup! {
                " Fixed "{Files(self.1)}"."
            })
        } else {
            fmt.write_markup(markup! {
                " No fixes applied."
            })
        }
    }
}
struct SummaryTotal<'a>(&'a TraversalMode, usize, &'a Duration);

impl fmt::Display for SummaryTotal<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let files = Files(self.1);
        match self.0 {
            TraversalMode::Check { .. } | TraversalMode::Lint { .. } | TraversalMode::CI { .. } => {
                fmt.write_markup(markup! {
                    "Checked "{files}" in "{self.2}"."
                })
            }
            TraversalMode::Format { write, .. } => {
                if *write {
                    fmt.write_markup(markup! {
                        "Formatted "{files}" in "{self.2}"."
                    })
                } else {
                    fmt.write_markup(markup! {
                        "Checked "{files}" in "{self.2}"."
                    })
                }
            }

            TraversalMode::Migrate { write, .. } => {
                if *write {
                    fmt.write_markup(markup! {
                      "Migrated your configuration file in "{self.2}"."
                    })
                } else {
                    fmt.write_markup(markup! {
                        "Checked your configuration file in "{self.2}"."
                    })
                }
            }

            TraversalMode::Search { .. } => fmt.write_markup(markup! {
                "Searched "{files}" in "{self.2}"."
            }),
        }
    }
}

pub(crate) struct ConsoleTraversalSummary<'a>(
    pub(crate) &'a TraversalMode,
    pub(crate) &'a TraversalSummary,
);
impl fmt::Display for ConsoleTraversalSummary<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let summary = SummaryTotal(self.0, self.1.changed + self.1.unchanged, &self.1.duration);
        let detail = SummaryDetail(self.0, self.1.changed);
        fmt.write_markup(markup!(<Info>{summary}{detail}</Info>))?;

        if self.1.errors > 0 {
            if self.1.errors == 1 {
                fmt.write_markup(markup!("\n"<Error>"Found "{self.1.errors}" error."</Error>))?;
            } else {
                fmt.write_markup(markup!("\n"<Error>"Found "{self.1.errors}" errors."</Error>))?;
            }
        }
        if self.1.warnings > 0 {
            if self.1.warnings == 1 {
                fmt.write_markup(markup!("\n"<Warn>"Found "{self.1.warnings}" warning."</Warn>))?;
            } else {
                fmt.write_markup(markup!("\n"<Warn>"Found "{self.1.warnings}" warnings."</Warn>))?;
            }
        }

        if let TraversalMode::Search { .. } = self.0 {
            if self.1.matches == 1 {
                fmt.write_markup(markup!(" "<Info>"Found "{self.1.matches}" match."</Info>))?
            } else {
                fmt.write_markup(markup!(" "<Info>"Found "{self.1.matches}" matches."</Info>))?
            };
        };
        Ok(())
    }
}
