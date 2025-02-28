use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::PrintGitHubDiagnostic;
use std::io;

pub(crate) struct GithubReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
}

impl Reporter for GithubReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload)?;
        Ok(())
    }
}
pub(crate) struct GithubReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for GithubReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        _summary: TraversalSummary,
    ) -> io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        diagnostics_payload: DiagnosticsPayload,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && diagnostics_payload.verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                } else if !diagnostics_payload.verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                }
            }
        }

        Ok(())
    }
}
