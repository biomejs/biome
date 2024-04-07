pub(crate) mod terminal;

use crate::cli_options::CliOptions;
use crate::execute::Execution;
use crate::CliDiagnostic;
use biome_diagnostics::{Error, Severity};
use std::io;
use std::time::Duration;

pub struct DiagnosticsPayload<'a> {
    diagnostics: Vec<Error>,
    verbose: bool,
    diagnostic_level: Severity,
    execution: &'a Execution,
}

/// A type that holds the result of the traversal
#[derive(Debug, Default)]
pub struct TraversalSummary {
    pub changed: usize,
    pub unchanged: usize,
    pub duration: Duration,
    pub errors: u32,
    pub warnings: u32,
    pub skipped: usize,
    pub suggested_fixes_skipped: u32,
    pub diagnostics_not_printed: u32,
}

/// When using this trait, the type that implements this trait is the one that holds the read-only information to pass around
pub trait Reporter {
    /// Writes the summary using the underling visitor
    fn write(&mut self, visitor: &mut dyn ReporterVisitor) -> io::Result<()>;
}

/// When using this trait, the type that implements this trait is the one that will **write** the data, ideally inside a buffer
pub trait ReporterVisitor {
    /// Writes the summary in the underling writer
    fn report_summary(
        &mut self,
        traversal_mode: &Execution,
        summary: &TraversalSummary,
    ) -> io::Result<()>;

    /// Writes a diagnostics
    fn report_diagnostics(&mut self, payload: &DiagnosticsPayload) -> io::Result<()>;
}

/// This function reports the result of a traversal
pub(crate) fn report<R, V>(
    reporter: &mut R,
    reporter_visitor: &mut V,
    execution: &Execution,
    cli_options: &CliOptions,
    traverse_result: &TraversalSummary,
) -> Result<(), CliDiagnostic>
where
    R: Reporter,
    V: ReporterVisitor,
{
    let count = traverse_result.changed + traverse_result.unchanged;

    reporter.write(reporter_visitor)?;

    let should_exit_on_warnings = traverse_result.warnings > 0 && cli_options.error_on_warnings;
    // Processing emitted error diagnostics, exit with a non-zero code
    if count.saturating_sub(traverse_result.skipped) == 0 && !cli_options.no_errors_on_unmatched {
        Err(CliDiagnostic::no_files_processed())
    } else if traverse_result.errors > 0 || should_exit_on_warnings {
        let category = execution.as_diagnostic_category();
        if should_exit_on_warnings {
            if execution.is_check_apply() {
                Err(CliDiagnostic::apply_warnings(category))
            } else {
                Err(CliDiagnostic::check_warnings(category))
            }
        } else if execution.is_check_apply() {
            Err(CliDiagnostic::apply_error(category))
        } else {
            Err(CliDiagnostic::check_error(category))
        }
    } else {
        Ok(())
    }
}
