use crate::cli_options::CliOptions;
use crate::execute::{Execution, TraversalMode};
use crate::reporter::{
    DiagnosticReporter, DiagnosticVisitor, DiagnosticsPayload, ReporterVisitor, SummaryReporter,
    SummaryResult, SummaryVisitor,
};
use crate::Reporter;
use biome_console::fmt::Formatter;
use biome_console::{fmt, markup, Console, ConsoleExt};
use biome_diagnostics::{Error, PrintDiagnostic, PrintGitHubDiagnostic, Severity};
use std::io;
use std::time::Duration;

pub(crate) struct ConsoleReporter<'a> {
    summary: &'a SummaryResult,
    diagnostics_payload: DiagnosticsPayload<'a>,
    execution: &'a Execution,
}

#[derive(Default)]
pub(crate) struct ConsoleReporterBuilder<'a> {
    cli_options: Option<&'a CliOptions>,
    execution: Option<&'a Execution>,
    summary: Option<&'a SummaryResult>,
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
    pub(crate) fn with_summary(mut self, summary_result: &'a SummaryResult) -> Self {
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
        summary: &SummaryResult,
    ) -> io::Result<()> {
        let mut visitor = ConsoleSummaryVisitor {
            console: self.0,
            summary,
            execution: traversal_mode,
        };
        let mut reporter = ConsoleSummaryReporter(summary);
        reporter.report(&mut visitor)?;
        Ok(())
    }

    fn report_diagnostics(&mut self, diagnostics_payload: &DiagnosticsPayload) -> io::Result<()> {
        let mut visitor = ConsoleDiagnosticVisitor { console: self.0 };
        let mut reporter = ConsoleDiagnosticReporter(diagnostics_payload);
        reporter.report(&mut visitor)?;
        Ok(())
    }
}

struct ConsoleSummaryVisitor<'a> {
    console: &'a mut dyn Console,
    summary: &'a SummaryResult,
    execution: &'a Execution,
}

struct ConsoleSummaryReporter<'a>(&'a SummaryResult);

impl<'a> SummaryReporter for ConsoleSummaryReporter<'a> {
    fn report(&mut self, visitor: &mut dyn SummaryVisitor) -> io::Result<()> {
        visitor.report_skipped_fixes(self.0.suggested_fixes_skipped as usize)?;
        visitor.report_not_printed_diagnostics(self.0.diagnostics_not_printed as usize)?;
        visitor.report_total(self.0.changed + self.0.unchanged)?;
        visitor.report_changed(self.0.changed)?;
        visitor.report_skipped(self.0.skipped)?;
        visitor.report_errors(self.0.errors as usize)?;
        visitor.report_warnings(self.0.warnings as usize)?;

        Ok(())
    }
}

impl<'a> SummaryVisitor for ConsoleSummaryVisitor<'a> {
    fn report_skipped_fixes(&mut self, skipped: usize) -> io::Result<()> {
        if self.execution.is_check() && skipped > 0 {
            self.console.log(markup! {
                <Warn>"Skipped "{skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
            })
        }
        Ok(())
    }

    fn report_not_printed_diagnostics(&mut self, not_printed: usize) -> io::Result<()> {
        if !self.execution.is_ci() && not_printed > 0 {
            self.console.log(markup! {
                <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{not_printed}</Emphasis><Info>"."</Info>
            })
        }
        Ok(())
    }
    fn report_total(&mut self, total: usize) -> io::Result<()> {
        let total = SummaryTotal(
            self.execution.traversal_mode(),
            total,
            &self.summary.duration,
        );
        self.console.log(markup! {
            <Info>{total}</Info>
        });

        Ok(())
    }

    fn report_changed(&mut self, changed: usize) -> io::Result<()> {
        let detail = SummaryDetail(changed);
        self.console.log(markup! {
            <Info>{detail}</Info>
        });
        Ok(())
    }

    fn report_skipped(&mut self, skipped: usize) -> io::Result<()> {
        if skipped > 0 {
            if skipped == 1 {
                self.console.log(markup! {
                    <Warn>"Skipped "{skipped}" file."</Warn>
                });
            } else {
                self.console.log(markup! {
                    <Warn>"Skipped "{skipped}" files."</Warn>
                });
            }
        }
        Ok(())
    }

    fn report_errors(&mut self, errors: usize) -> io::Result<()> {
        if errors > 0 {
            if errors == 1 {
                self.console
                    .log(markup!(<Error>"Found "{errors}" error."</Error>));
            } else {
                self.console
                    .log(markup!(<Error>"Found "{errors}" errors."</Error>));
            }
        }

        Ok(())
    }

    fn report_warnings(&mut self, warnings: usize) -> io::Result<()> {
        if warnings > 0 {
            if warnings == 1 {
                self.console
                    .log(markup!(<Warn>"Found "{warnings}" warning."</Warn>));
            } else {
                self.console
                    .log(markup!(<Warn>"Found "{warnings}" warnings."</Warn>));
            }
        }

        Ok(())
    }
}

struct ConsoleDiagnosticVisitor<'a> {
    console: &'a mut dyn Console,
}

struct ConsoleDiagnosticReporter<'a>(&'a DiagnosticsPayload<'a>);

impl<'a> DiagnosticReporter for ConsoleDiagnosticReporter<'a> {
    fn report(&mut self, visitor: &mut dyn DiagnosticVisitor) -> io::Result<()> {
        for diagnostic in &self.0.diagnostics {
            visitor.report_diagnostic(
                diagnostic,
                self.0.verbose,
                self.0.diagnostic_level,
                self.0.execution,
            )?;
        }

        Ok(())
    }
}

impl<'a> DiagnosticVisitor for ConsoleDiagnosticVisitor<'a> {
    fn report_diagnostic(
        &mut self,
        diagnostic: &Error,
        verbose: bool,
        diagnostic_level: Severity,
        execution: &Execution,
    ) -> io::Result<()> {
        if diagnostic.severity() >= diagnostic_level {
            if diagnostic.tags().is_verbose() && verbose {
                self.console
                    .error(markup! {{PrintDiagnostic::verbose(diagnostic)}});
            } else {
                self.console
                    .error(markup! {{PrintDiagnostic::simple(diagnostic)}});
            }
        }
        if execution.is_ci_github() {
            self.console
                .log(markup! {{PrintGitHubDiagnostic::simple(diagnostic)}});
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
