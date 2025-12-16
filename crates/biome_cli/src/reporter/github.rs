use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::PrintGitHubDiagnostic;
use camino::{Utf8Path, Utf8PathBuf};
use std::io;

pub(crate) struct GithubReporter<'a> {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for GithubReporter<'_> {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        Ok(())
    }
}
pub(crate) struct GithubReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for GithubReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &dyn Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &dyn Execution,
        diagnostics_payload: DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                } else if !verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                }
            }
        }

        Ok(())
    }
}
