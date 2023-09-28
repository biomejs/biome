use std::io;

use biome_console::fmt;

use crate::display::frame::SourceFile;
use crate::{diagnostic::internal::AsDiagnostic, Diagnostic, Resource, Severity};

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [biome_console::fmt::Write].
pub struct PrintGitHubDiagnostic<'fmt, D: ?Sized> {
    diag: &'fmt D,
}

impl<'fmt, D: AsDiagnostic + ?Sized> PrintGitHubDiagnostic<'fmt, D> {
    pub fn simple(diag: &'fmt D) -> Self {
        Self { diag }
    }
}

impl<'fmt, D: AsDiagnostic + ?Sized> fmt::Display for PrintGitHubDiagnostic<'fmt, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let diagnostic = self.diag.as_diagnostic();
        let location = diagnostic.location();

        // Docs:
        // https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions

        let Some(span) = location.span else {
            return Ok(());
        };
        let Some(source_code) = location.source_code else {
            return Ok(());
        };

        // TODO: Escape file names
        let file_name = match &location.resource {
            Some(Resource::File(file)) => file,
            _ => return Ok(()),
        };

        let source = SourceFile::new(source_code);
        let start = source.location(span.start())?;
        let end = source.location(span.end())?;

        let title = "TODO"; // TODO

        let prefix = match diagnostic.severity() {
            Severity::Error | Severity::Fatal => "error",
            Severity::Warning => "warning",
            Severity::Hint => "notice",
            Severity::Information => return Ok(()),
        };

        fmt.write_str(
            format! {
                "::{} file={},line={},endLine={},col={},endColumn={}::{}",
                prefix,
                file_name,
                start.line_number,
                end.line_number,
                start.column_number,
                end.column_number,
                title,
            }
            .as_str(),
        )?;

        Ok(())
    }
}
