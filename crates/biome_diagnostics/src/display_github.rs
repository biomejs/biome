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

        let file_name_unescaped = match &location.resource {
            Some(Resource::File(file)) => file,
            _ => return Ok(()),
        };

        let source = SourceFile::new(source_code);
        let start = source.location(span.start())?;
        let end = source.location(span.end())?;

        let title = "TODO"; // TODO

        let command = match diagnostic.severity() {
            Severity::Error | Severity::Fatal => "error",
            Severity::Warning => "warning",
            Severity::Hint => "notice",
            Severity::Information => return Ok(()),
        };

        fmt.write_str(
            format! {
                "::{} file={},line={},endLine={},col={},endColumn={}::{}",
                command, // constant, doesn't need escaping
                escape_property(file_name_unescaped),
                start.line_number, // integer, doesn't need escaping
                end.line_number, // integer, doesn't need escaping
                start.column_number, // integer, doesn't need escaping
                end.column_number, // integer, doesn't need escaping
                escape_data(title),
            }
            .as_str(),
        )?;

        Ok(())
    }
}

fn escape_data<S: AsRef<str>>(value: S) -> String {
    let value = value.as_ref();

    // Refs:
    // - https://github.com/actions/runner/blob/a4c57f27477077e57545af79851551ff7f5632bd/src/Runner.Common/ActionCommand.cs#L18-L22
    // - https://github.com/actions/toolkit/blob/fe3e7ce9a7f995d29d1fcfd226a32bca407f9dc8/packages/core/src/command.ts#L80-L94
    let mut result = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\r' => result.push_str("%0D"),
            '\n' => result.push_str("%0A"),
            '%' => result.push_str("%25"),
            _ => result.push(c),
        }
    }
    result
}

fn escape_property<S: AsRef<str>>(value: S) -> String {
    let value = value.as_ref();

    // Refs:
    // - https://github.com/actions/runner/blob/a4c57f27477077e57545af79851551ff7f5632bd/src/Runner.Common/ActionCommand.cs#L25-L32
    // - https://github.com/actions/toolkit/blob/fe3e7ce9a7f995d29d1fcfd226a32bca407f9dc8/packages/core/src/command.ts#L80-L94
    let mut result = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\r' => result.push_str("%0D"),
            '\n' => result.push_str("%0A"),
            ':' => result.push_str("%3A"),
            ',' => result.push_str("%2C"),
            '%' => result.push_str("%25"),
            _ => result.push(c),
        }
    }
    result
}
