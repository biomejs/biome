use crate::cli_options::CliOptions;
use crate::execute::{Execution, TraversalMode};
use crate::reporter::{DiagnosticsPayload, ReporterVisitor, TraversalSummary};
use crate::Reporter;
use biome_console::fmt::Formatter;
use biome_console::{fmt, markup, Console, ConsoleExt};
use biome_diagnostics::{Error, PrintDiagnostic, PrintGitHubDiagnostic};
use std::io;
use std::time::Duration;

pub(crate) struct ConsoleReporter<'a> {
    summary: &'a TraversalSummary,
    diagnostics_payload: DiagnosticsPayload<'a>,
    execution: &'a Execution,
}

#[derive(Default)]
pub(crate) struct ConsoleReporterBuilder<'a> {
    cli_options: Option<&'a CliOptions>,
    execution: Option<&'a Execution>,
    summary: Option<&'a TraversalSummary>,
    diagnostics: Vec<Error>,
}

impl<'a> ConsoleReporterBuilder<'a> {
    pub(crate) fn with_cli_options(mut self, cli_options: &'a CliOptions) -> Self {
        self.cli_options = Some(cli_options);
        self
    }

    pub(crate) fn with_execution(mut self, execution: &'a Execution) -> Self {
        self.execution = Some(execution);
        self
    }
    pub(crate) fn with_summary(mut self, summary_result: &'a TraversalSummary) -> Self {
        self.summary = Some(summary_result);
        self
    }

    pub(crate) fn with_diagnostics(mut self, diagnostics: Vec<Error>) -> Self {
        self.diagnostics = diagnostics;
        self
    }

    pub(crate) fn finish(self) -> ConsoleReporter<'a> {
        let cli_options = self.cli_options.expect("to call with_cli_options()");
        let summary = self.summary.expect("to call with_summary()");
        let execution = self.execution.expect("to call with_traversal()");
        let diagnostics_payload = DiagnosticsPayload {
            execution,
            verbose: cli_options.verbose,
            diagnostic_level: cli_options.diagnostic_level,
            diagnostics: self.diagnostics,
        };

        ConsoleReporter {
            summary,
            diagnostics_payload,
            execution,
        }
    }
}
impl<'a> Reporter for ConsoleReporter<'a> {
    fn write(&mut self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(&self.diagnostics_payload)?;
        visitor.report_summary(self.execution, self.summary)?;
        Ok(())
    }
}
pub(crate) struct ConsoleReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl<'a> ReporterVisitor for ConsoleReporterVisitor<'a> {
    fn report_summary(
        &mut self,
        traversal_mode: &Execution,
        summary: &TraversalSummary,
    ) -> io::Result<()> {
        self.report_skipped_fixes(traversal_mode, summary.suggested_fixes_skipped as usize)?;
        self.report_not_printed_diagnostics(
            traversal_mode,
            summary.diagnostics_not_printed as usize,
        )?;
        self.report_total(
            traversal_mode,
            summary.changed() + summary.unchanged(),
            summary.duration(),
        )?;
        self.report_changed(traversal_mode, summary.changed)?;
        self.report_skipped(traversal_mode, summary.skipped)?;
        self.report_errors(traversal_mode, summary.errors as usize)?;
        self.report_warnings(traversal_mode, summary.warnings as usize)?;
        Ok(())
    }

    fn report_diagnostics(&mut self, diagnostics_payload: &DiagnosticsPayload) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && diagnostics_payload.verbose {
                    self.0
                        .error(markup! {{PrintDiagnostic::verbose(diagnostic)}});
                } else {
                    self.0
                        .error(markup! {{PrintDiagnostic::simple(diagnostic)}});
                }
            }
            if diagnostics_payload.execution.is_ci_github() {
                self.0
                    .log(markup! {{PrintGitHubDiagnostic::simple(diagnostic)}});
            }
        }

        Ok(())
    }

    fn report_skipped_fixes(&mut self, execution: &Execution, skipped: usize) -> io::Result<()> {
        if execution.is_check() && skipped > 0 {
            self.0.log(markup! {
                <Warn>"Skipped "{skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
            })
        }
        Ok(())
    }

    fn report_not_printed_diagnostics(
        &mut self,
        execution: &Execution,
        not_printed: usize,
    ) -> io::Result<()> {
        if !execution.is_ci() && not_printed > 0 {
            self.0.log(markup! {
                <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{not_printed}</Emphasis><Info>"."</Info>
            })
        }
        Ok(())
    }
    fn report_total(
        &mut self,
        execution: &Execution,
        total: usize,
        duration: Duration,
    ) -> io::Result<()> {
        let total = SummaryTotal(execution.traversal_mode(), total, &duration);
        self.0.log(markup! {
            <Info>{total}</Info>
        });

        Ok(())
    }

    fn report_changed(&mut self, _execution: &Execution, changed: usize) -> io::Result<()> {
        let detail = SummaryDetail(changed);
        self.0.log(markup! {
            <Info>{detail}</Info>
        });
        Ok(())
    }

    fn report_skipped(&mut self, _execution: &Execution, skipped: usize) -> io::Result<()> {
        if skipped > 0 {
            if skipped == 1 {
                self.0.log(markup! {
                    <Warn>"Skipped "{skipped}" file."</Warn>
                });
            } else {
                self.0.log(markup! {
                    <Warn>"Skipped "{skipped}" files."</Warn>
                });
            }
        }
        Ok(())
    }

    fn report_errors(&mut self, _execution: &Execution, errors: usize) -> io::Result<()> {
        if errors > 0 {
            if errors == 1 {
                self.0
                    .log(markup!(<Error>"Found "{errors}" error."</Error>));
            } else {
                self.0
                    .log(markup!(<Error>"Found "{errors}" errors."</Error>));
            }
        }

        Ok(())
    }

    fn report_warnings(&mut self, _execution: &Execution, warnings: usize) -> io::Result<()> {
        if warnings > 0 {
            if warnings == 1 {
                self.0
                    .log(markup!(<Warn>"Found "{warnings}" warning."</Warn>));
            } else {
                self.0
                    .log(markup!(<Warn>"Found "{warnings}" warnings."</Warn>));
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

struct SummaryDetail(usize);

impl fmt::Display for SummaryDetail {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if self.0 > 0 {
            fmt.write_markup(markup! {
                "Fixed "{Files(self.0)}"."
            })
        } else {
            fmt.write_markup(markup! {
                "No fixes needed."
            })
        }
    }
}
struct SummaryTotal<'a>(&'a TraversalMode, usize, &'a Duration);

impl<'a> fmt::Display for SummaryTotal<'a> {
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
