use crate::display::frame::SourceFile;
use crate::{diagnostic::internal::AsDiagnostic, Diagnostic, Resource, Severity};
use biome_console::{fmt, markup, MarkupBuf};
use biome_text_size::{TextRange, TextSize};
use std::io;

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [biome_console::fmt::Write].
pub struct PrintGitHubDiagnostic<'fmt, D: ?Sized>(pub &'fmt D);

impl<D: AsDiagnostic + ?Sized> fmt::Display for PrintGitHubDiagnostic<'_, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let diagnostic = self.0.as_diagnostic();
        let location = diagnostic.location();

        // Docs:
        // https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions
        let span = location
            .span
            // We fall back to 1:1. This usually covers diagnostics that belong to the formatter or organize imports
            .unwrap_or(TextRange::new(TextSize::from(1), TextSize::from(1)));

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

        let command = match diagnostic.severity() {
            Severity::Error | Severity::Fatal => "error",
            Severity::Warning => "warning",
            Severity::Hint | Severity::Information => "notice",
        };

        let message = {
            let mut message = MarkupBuf::default();
            let mut fmt = fmt::Formatter::new(&mut message);
            fmt.write_markup(markup!({ PrintDiagnosticMessage(diagnostic) }))?;
            markup_to_string(&message)
        };

        let title = {
            diagnostic
                .category()
                .map(|category| category.name())
                .unwrap_or_default()
        };

        fmt.write_str(
            format! {
                "::{} title={},file={},line={},endLine={},col={},endColumn={}::{}",
                command, // constant, doesn't need escaping
                title, // the diagnostic category
                escape_property(file_name_unescaped),
                start.line_number, // integer, doesn't need escaping
                end.line_number, // integer, doesn't need escaping
                start.column_number, // integer, doesn't need escaping
                end.column_number, // integer, doesn't need escaping
                message.map_or_else(String::new, escape_data),
            }
            .as_str(),
        )?;

        Ok(())
    }
}

struct PrintDiagnosticMessage<'fmt, D: ?Sized>(&'fmt D);

impl<D: Diagnostic + ?Sized> fmt::Display for PrintDiagnosticMessage<'_, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let Self(diagnostic) = *self;
        diagnostic.message(fmt)?;
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

fn markup_to_string(markup: &MarkupBuf) -> Option<String> {
    let mut buffer = Vec::new();
    let mut write = fmt::Termcolor(termcolor::NoColor::new(&mut buffer));
    let mut fmt = fmt::Formatter::new(&mut write);
    fmt.write_markup(markup! { {markup} }).ok()?;
    String::from_utf8(buffer).ok()
}
