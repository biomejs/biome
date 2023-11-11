use crate::cli_options::CliOptions;
use crate::execute::{Execution, ReportMode, TraversalMode};
use crate::CliDiagnostic;
use biome_console::{markup, Console, ConsoleExt, Markup};
use biome_diagnostics::{Error, PrintDiagnostic, Severity};
use std::time::Duration;
use tracing::error;
use tracing::log::Log;

/// Information computed from a diff result
#[derive(Debug)]
pub struct ReportDiff {
    /// The severity fo the diff
    pub severity: Severity,
    /// How was the code before the command
    pub before: String,
    /// How is the code after the command
    pub after: String,
}

#[derive(Debug)]
pub enum ReportKind {
    Diagnostic(Error),
    Diff(ReportDiff),
}

pub trait Reporter: Send + Sync // where
//     W: ReportWriter,
{
    // fn report_message(&mut self, message: impl Display);

    fn report_not_printed_diagnostics(&mut self, number: u64);
    fn report_skipped_fixes(&mut self, number: u32);
    fn report_diagnostic(&mut self, diagnostic: Error);

    fn report_diff(&mut self, path: String, report: ReportDiff);

    fn finish(self, execution: Execution) -> Dumper;
}

pub trait ReportWriter {
    type Formatter;

    fn error(&mut self, markup: Markup) -> Result<(), CliDiagnostic>;

    fn out(&mut self) -> Result<(), CliDiagnostic>;
}

pub struct ConsoleReporter {
    pub(crate) mode: ReportMode,
    pub(crate) diagnostics: Vec<Error>,
    pub(crate) verbose: bool,
    pub(crate) not_reported: u64,
    pub(crate) skipped_fixes: u32,
}

impl ConsoleReporter {
    pub fn new(verbose: bool, mode: ReportMode) -> Self {
        Self {
            mode,
            verbose,
            diagnostics: Vec::default(),
            not_reported: 0,
            skipped_fixes: 0,
        }
    }
}

pub struct Dumper {
    pub skipped: usize,
    pub errors: usize,
    pub warnings: usize,
    pub count: usize,
    pub diagnostics: Vec<Error>,
    pub execution: Execution,
    pub duration: Duration,
    pub not_printed_diagnostics: u64,
    pub skipped_suggested_fixes: u32,
}

impl Dumper {
    pub fn new(
        execution: Execution,
        diagnostics: Vec<Error>,
        not_printed_diagnostics: u64,
        skipped_suggested_fixes: u32,
    ) -> Self {
        Self {
            execution,
            skipped: 0,
            errors: 0,
            warnings: 0,
            count: 0,
            diagnostics,
            duration: Duration::default(),
            not_printed_diagnostics,
            skipped_suggested_fixes,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
    pub fn with_skipped(mut self, skipped: usize) -> Self {
        self.skipped = skipped;
        self
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
    pub fn with_warnings(mut self, warnings: usize) -> Self {
        self.warnings = warnings;
        self
    }
    pub fn with_errors(mut self, errors: usize) -> Self {
        self.errors = errors;
        self
    }

    pub fn dump(
        self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
    ) -> Result<(), CliDiagnostic> {
        for diagnostic in &self.diagnostics {
            console.error(markup! {
                // TODO: verbosity
                {if true { PrintDiagnostic::verbose(diagnostic) } else { PrintDiagnostic::simple(diagnostic) }}
            });
        }

        if self.execution.is_check() && self.skipped_suggested_fixes > 0 {
            console.log(markup! {
                <Warn>"Skipped "{self.skipped_suggested_fixes}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
            })
        }

        if !self.execution.is_ci() && self.not_printed_diagnostics > 0 {
            console.log(markup! {
                <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{self.not_printed_diagnostics}</Emphasis><Info>"."</Info>
            })
        }

        match self.execution.traversal_mode() {
            TraversalMode::Check { .. } | TraversalMode::Lint { .. } => {
                if self.execution.as_fix_file_mode().is_some() {
                    console.log(markup! {
                        <Info>"Fixed "{self.count}" file(s) in "{self.duration}</Info>
                    });
                } else {
                    console.log(
                        markup!(<Info>"Checked "{self.count}" file(s) in "{self.duration}</Info>),
                    );

                    if self.errors > 0 {
                        console.log(markup!("\n"<Error>"Found "{self.errors}" error(s)"</Error>))
                    }
                }
            }
            TraversalMode::CI { .. } => {
                console
                    .log(markup!(<Info>"Checked "{self.count}" file(s) in "{self.duration}</Info>));

                if self.errors > 0 {
                    console.log(markup!("\n"<Error>"Found "{self.errors}" error(s)"</Error>))
                }
            }
            TraversalMode::Format { write: false, .. } => {
                console.log(markup! {
                    <Info>"Compared "{self.count}" file(s) in "{self.duration}</Info>
                });
            }
            TraversalMode::Format { write: true, .. } => {
                console.log(markup! {
                    <Info>"Formatted "{self.count}" file(s) in "{self.duration}</Info>
                });
            }

            TraversalMode::Migrate { write: false, .. } => {
                console.log(markup! {
                    <Info>"Checked your configuration file in "{self.duration}</Info>
                });
            }

            TraversalMode::Migrate { write: true, .. } => {
                console.log(markup! {
                    <Info>"Migrated your configuration file in "{self.duration}</Info>
                });
            }
        }

        if self.skipped > 0 {
            console.log(markup! {
                <Warn>"Skipped "{self.skipped}" file(s)"</Warn>
            });
        }

        let should_exit_on_warnings = self.warnings > 0 && cli_options.error_on_warnings;
        // Processing emitted error diagnostics, exit with a non-zero code
        if self.count.saturating_sub(self.skipped) == 0 && !cli_options.no_errors_on_unmatched {
            Err(CliDiagnostic::no_files_processed())
        } else if self.errors > 0 || should_exit_on_warnings {
            let category = self.execution.as_diagnostic_category();
            if should_exit_on_warnings {
                if self.execution.is_check_apply() {
                    Err(CliDiagnostic::apply_warnings(category))
                } else {
                    Err(CliDiagnostic::check_warnings(category))
                }
            } else if self.execution.is_check_apply() {
                Err(CliDiagnostic::apply_error(category))
            } else {
                Err(CliDiagnostic::check_error(category))
            }
        } else {
            Ok(())
        }
    }
}

impl Reporter for ConsoleReporter {
    fn report_not_printed_diagnostics(&mut self, number: u64) {
        self.not_reported = number;
    }

    fn report_skipped_fixes(&mut self, number: u32) {
        self.skipped_fixes = number;
    }

    fn report_diagnostic(&mut self, diagnostic: Error) {
        self.diagnostics.push(diagnostic)
    }

    fn report_diff(&mut self, path: String, report: ReportDiff) {
        todo!()
    }

    fn finish(self, execution: Execution) -> Dumper {
        Dumper::new(
            execution,
            self.diagnostics,
            self.not_reported,
            self.skipped_fixes,
        )
    }
}
