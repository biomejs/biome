use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::PrintGitHubDiagnostic;
use std::io;

pub(crate) struct GithubReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) verbose: bool,
}

impl Reporter for GithubReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload, self.verbose)?;
        Ok(())
    }
}
pub(crate) struct GithubReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for GithubReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        diagnostics_payload: DiagnosticsPayload,
        verbose: bool,
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
