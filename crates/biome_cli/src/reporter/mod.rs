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

pub struct SummaryResult {
    pub(crate) changed: usize,
    pub(crate) unchanged: usize,
    pub(crate) duration: Duration,
    pub(crate) errors: u32,
    pub(crate) warnings: u32,
    pub(crate) skipped: usize,
    pub(crate) suggested_fixes_skipped: u32,
    pub(crate) diagnostics_not_printed: u32,
}

pub trait Reporter {
    /// Writes the summary using the underling visitor
    fn write(&mut self, visitor: &mut dyn ReporterVisitor) -> io::Result<()>;
}

// pub trait TerminalReporter: Reporter {
//     fn dump_to_terminal(&self, console: &mut dyn Console, visitor: &mut dyn ReporterVisitor);
// }

pub trait ReporterVisitor {
    /// Writes the summary in the underling writer
    fn report_summary(
        &mut self,
        traversal_mode: &Execution,
        summary: &SummaryResult,
    ) -> io::Result<()> {
        let _ = (summary, traversal_mode);
        Ok(())
    }

    /// Writes a diagnostic
    fn report_diagnostics(&mut self, payload: &DiagnosticsPayload) -> io::Result<()> {
        let _ = payload;
        Ok(())
    }
}

trait SummaryReporter {
    fn report(&mut self, visitor: &mut dyn SummaryVisitor) -> io::Result<()>;
}

trait SummaryVisitor {
    /// Reports the total files that were checked
    fn report_skipped_fixes(&mut self, skipped: usize) -> io::Result<()>;

    /// Reports the total files that were checked
    fn report_not_printed_diagnostics(&mut self, not_printed: usize) -> io::Result<()>;
    /// Reports the total files that were checked
    fn report_total(&mut self, total: usize) -> io::Result<()>;

    /// Reports the number of files that were changed/modified
    fn report_changed(&mut self, changed: usize) -> io::Result<()>;

    /// Reports the number of files that were skipped
    fn report_skipped(&mut self, skipped: usize) -> io::Result<()>;

    /// Reports the number of errors emitted during the traversal
    fn report_errors(&mut self, errors: usize) -> io::Result<()>;

    /// Reports the number of warnings emitted during the traversal
    fn report_warnings(&mut self, warnings: usize) -> io::Result<()>;
}

trait DiagnosticReporter {
    fn report(&mut self, visitor: &mut dyn DiagnosticVisitor) -> io::Result<()>;
}

trait DiagnosticVisitor {
    /// Reports a single diagnostic. It receives the following information:
    /// - `verbose`: if the diagnostic should be printed in verbose mode
    /// - `diagnostic_level`: the minimum level of severity requested
    /// - `ci_kind`: the kind of CI environment, if any
    fn report_diagnostic(
        &mut self,
        diagnostic: &Error,
        verbose: bool,
        diagnostic_level: Severity,
        execution: &Execution,
    ) -> io::Result<()>;
}

pub(crate) fn report<R, V>(
    reporter: &mut R,
    reporter_visitor: &mut V,
    execution: &Execution,
    cli_options: &CliOptions,
    traverse_result: &SummaryResult,
) -> Result<(), CliDiagnostic>
where
    R: Reporter,
    V: ReporterVisitor,
{
    let count = traverse_result.changed + traverse_result.unchanged;

    // TODO: handle error properly
    reporter.write(reporter_visitor).unwrap();

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
