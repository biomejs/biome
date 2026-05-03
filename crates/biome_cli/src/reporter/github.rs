use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::markup;
use biome_diagnostics::PrintGitHubDiagnostic;
use camino::{Utf8Path, Utf8PathBuf};
use std::io;

pub(crate) struct GithubReporter<'a> {
    pub diagnostics_payload: &'a DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for GithubReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,

        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()> {
        visitor.report_diagnostics(
            writer,
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        Ok(())
    }
}
pub(crate) struct GithubReporterVisitor;

impl ReporterVisitor for GithubReporterVisitor {
    fn report_summary(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        diagnostics_payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if !diagnostic.tags().is_verbose() {
                    writer.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                } else if diagnostic.tags().is_verbose() && verbose {
                    writer.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                }
            }
        }

        Ok(())
    }
}
