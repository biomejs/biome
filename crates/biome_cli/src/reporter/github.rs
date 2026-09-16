use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::markup;
use biome_diagnostics::{PrintGitHubDiagnostic, Resource};
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
        working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level
                && (!diagnostic.tags().is_verbose() || verbose)
            {
                let file_path = match diagnostic.location().resource {
                    Some(Resource::File(file_path)) if Utf8Path::new(file_path).is_relative() => {
                        working_directory.map(|working_directory| working_directory.join(file_path))
                    }
                    _ => None,
                };
                let diagnostic = PrintGitHubDiagnostic(diagnostic);
                if let Some(file_path) = file_path {
                    writer.log(markup! {{diagnostic.with_file_path(file_path.as_str())}});
                } else {
                    writer.log(markup! {{diagnostic}});
                }
            }
        }

        Ok(())
    }
}
