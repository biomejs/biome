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
    changed: usize,
    unchanged: usize,
    duration: Duration,
    errors: u32,
    warnings: u32,
    skipped: usize,
    suggested_fixes_skipped: u32,
    diagnostics_not_printed: u32,
}

impl TraversalSummary {
    pub fn with_changed(mut self, value: usize) -> Self {
        self.changed = value;
        self
    }
    pub fn with_unchanged(mut self, value: usize) -> Self {
        self.unchanged = value;
        self
    }
    pub fn with_duration(mut self, value: Duration) -> Self {
        self.duration = value;
        self
    }
    pub fn with_errors(mut self, value: u32) -> Self {
        self.errors = value;
        self
    }

    pub fn with_warnings(mut self, value: u32) -> Self {
        self.warnings = value;
        self
    }

    pub fn with_skipped(mut self, value: usize) -> Self {
        self.skipped = value;
        self
    }

    pub fn with_suggested_fixes_skipped(mut self, value: u32) -> Self {
        self.suggested_fixes_skipped = value;
        self
    }

    pub fn with_diagnostics_not_printed(mut self, value: u32) -> Self {
        self.diagnostics_not_printed = value;
        self
    }

    pub fn changed(&self) -> usize {
        self.changed
    }
    pub fn unchanged(&self) -> usize {
        self.unchanged
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn errors(&self) -> u32 {
        self.errors
    }
    pub fn warnings(&self) -> u32 {
        self.warnings
    }
    pub fn skipped(&self) -> usize {
        self.skipped
    }
    pub fn suggested_fixes_skipped(&self) -> u32 {
        self.suggested_fixes_skipped
    }
    pub fn diagnostics_not_printed(&self) -> u32 {
        self.diagnostics_not_printed
    }
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
    ) -> io::Result<()> {
        let _ = (summary, traversal_mode);
        Ok(())
    }

    /// Writes a diagnostic
    fn report_diagnostics(&mut self, payload: &DiagnosticsPayload) -> io::Result<()> {
        let _ = payload;
        Ok(())
    }

    /// Reports the fixes that were skipped
    fn report_skipped_fixes(&mut self, execution: &Execution, skipped: usize) -> io::Result<()> {
        let _ = (execution, skipped);
        Ok(())
    }

    /// Reports diagnostics that were not printed
    fn report_not_printed_diagnostics(
        &mut self,
        execution: &Execution,
        not_printed: usize,
    ) -> io::Result<()> {
        let _ = (execution, not_printed);
        Ok(())
    }

    /// Reports the total files that were checked
    fn report_total(
        &mut self,
        execution: &Execution,
        total: usize,
        duration: Duration,
    ) -> io::Result<()> {
        let _ = (execution, total, duration);
        Ok(())
    }

    /// Reports the number of files that were changed/modified
    fn report_changed(&mut self, execution: &Execution, changed: usize) -> io::Result<()> {
        let _ = (execution, changed);
        Ok(())
    }

    /// Reports the number of files that were skipped
    fn report_skipped(&mut self, execution: &Execution, skipped: usize) -> io::Result<()> {
        let _ = (execution, skipped);
        Ok(())
    }

    /// Reports the number of errors emitted during the traversal
    fn report_errors(&mut self, execution: &Execution, errors: usize) -> io::Result<()> {
        let _ = (execution, errors);
        Ok(())
    }

    /// Reports the number of warnings emitted during the traversal
    fn report_warnings(&mut self, execution: &Execution, warnings: usize) -> io::Result<()> {
        let _ = (execution, warnings);
        Ok(())
    }
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
