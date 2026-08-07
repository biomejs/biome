use anyhow::Result;
use biome_console::{
    Console,
    fmt::{Formatter, Write},
    markup,
};
use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity, Visit};
use biome_json_syntax::TextSize;
use biome_text_edit::TextEdit;
use std::io;

/// Receives diagnostics and code actions produced from rule documentation.
pub trait DiagnosticWriter {
    fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) -> Result<()>;
    fn write_parse_error(&mut self, diag: biome_diagnostics::Error) -> Result<()>;
    fn write_action(&mut self, source: &str, file_path: &str, edit: TextEdit) -> Result<()>;
    fn print_all_diagnostics(&mut self) -> Result<()>;
    fn subtract_offset(&mut self, offset: TextSize);
}

#[derive(Default)]
pub struct DiagnosticConsoleWriter {
    pub all_diagnostics: Vec<biome_diagnostics::Error>,
    pub action_count: usize,
    pub has_parse_error: bool,
    pub subtract_offset: TextSize,
}

impl DiagnosticWriter for DiagnosticConsoleWriter {
    fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) -> Result<()> {
        self.all_diagnostics.push(self.adjust_span_offset(diag));
        Ok(())
    }

    fn write_parse_error(&mut self, diag: biome_diagnostics::Error) -> Result<()> {
        self.has_parse_error = true;
        self.write_diagnostic(diag)
    }

    fn write_action(&mut self, _source: &str, _file_path: &str, _edit: TextEdit) -> Result<()> {
        self.action_count += 1;
        Ok(())
    }

    fn print_all_diagnostics(&mut self) -> Result<()> {
        let mut console = biome_console::EnvConsole::default();
        for diag in &self.all_diagnostics {
            console.println(
                biome_console::LogLevel::Error,
                markup! {
                    {PrintDiagnostic::verbose(diag)}
                },
            );
        }
        Ok(())
    }

    fn subtract_offset(&mut self, offset: TextSize) {
        self.subtract_offset = offset;
    }
}

#[derive(Clone, Copy, Debug)]
/// Selects the output rendered by [`DiagnosticHtmlWriter`].
pub enum DiagnosticHtmlWriterMode {
    Diagnostics,
    Actions,
}

/// Renders rule diagnostics or code actions as HTML.
pub struct DiagnosticHtmlWriter<'a> {
    buffer: &'a mut dyn Write,
    mode: DiagnosticHtmlWriterMode,
    subtract_offset: TextSize,
}

impl<'a> DiagnosticHtmlWriter<'a> {
    /// Creates a writer that renders the selected output mode into `buffer`.
    pub fn new(buffer: &'a mut dyn Write, mode: DiagnosticHtmlWriterMode) -> Self {
        Self {
            buffer,
            mode,
            subtract_offset: TextSize::default(),
        }
    }

    fn write_diagnostic_inner(&mut self, diag: biome_diagnostics::Error) -> Result<()> {
        let diag = adjust_span_offset(diag, self.subtract_offset);
        Formatter::new(self.buffer).write_markup(markup! {
            {PrintDiagnostic::verbose(&diag)}
        })?;
        Ok(())
    }
}

impl DiagnosticWriter for DiagnosticHtmlWriter<'_> {
    fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) -> Result<()> {
        if matches!(self.mode, DiagnosticHtmlWriterMode::Diagnostics) {
            self.write_diagnostic_inner(diag)?;
        }
        Ok(())
    }

    fn write_parse_error(&mut self, diag: biome_diagnostics::Error) -> Result<()> {
        self.write_diagnostic_inner(diag)
    }

    fn write_action(&mut self, source: &str, file_path: &str, edit: TextEdit) -> Result<()> {
        if matches!(self.mode, DiagnosticHtmlWriterMode::Actions) {
            let action = CodeAction(edit)
                .with_file_source_code(source)
                .with_file_path(file_path);
            Formatter::new(self.buffer).write_markup(markup! {
                {PrintDiagnostic::simple(&action)}
            })?;
        }
        Ok(())
    }

    fn print_all_diagnostics(&mut self) -> Result<()> {
        Ok(())
    }

    fn subtract_offset(&mut self, offset: TextSize) {
        self.subtract_offset = offset;
    }
}

#[derive(Debug)]
struct CodeAction(TextEdit);

impl Diagnostic for CodeAction {
    fn message(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup!("Source action diff:"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_diff(&self.0)
    }
}

impl DiagnosticConsoleWriter {
    /// Adjusts the location of the diagnostic to account for synthetic nodes
    /// that aren't present in the source code but only in the AST.
    pub fn adjust_span_offset(&self, diag: biome_diagnostics::Error) -> biome_diagnostics::Error {
        adjust_span_offset(diag, self.subtract_offset)
    }
}

fn adjust_span_offset(
    diag: biome_diagnostics::Error,
    subtract_offset: TextSize,
) -> biome_diagnostics::Error {
    if subtract_offset != 0.into() {
        if let Some(span) = diag.location().span {
            let new_span = span.checked_sub(subtract_offset);
            diag.with_file_span(new_span)
        } else {
            diag
        }
    } else {
        diag
    }
}
