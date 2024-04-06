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
        execution: &Execution,
        summary: &TraversalSummary,
    ) -> io::Result<()> {
        if execution.is_check() && summary.suggested_fixes_skipped > 0 {
            self.0.log(markup! {
                <Warn>"Skipped "{summary.suggested_fixes_skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
            })
        }

        if !execution.is_ci() && summary.diagnostics_not_printed > 0 {
            self.0.log(markup! {
                <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{summary.diagnostics_not_printed}</Emphasis><Info>"."</Info>
            })
        }

        self.0.log(markup! {
            {ConsoleTraversalSummary(execution.traversal_mode(), summary)}
        });

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
                " Fixed "{Files(self.0)}"."
            })
        } else {
            fmt.write_markup(markup! {
                " No fixes needed."
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

struct ConsoleTraversalSummary<'a>(pub(crate) &'a TraversalMode, &'a TraversalSummary);
impl<'a> fmt::Display for ConsoleTraversalSummary<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let summary = SummaryTotal(self.0, self.1.changed + self.1.unchanged, &self.1.duration);
        let detail = SummaryDetail(self.1.changed);
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
        Ok(())
    }
}
