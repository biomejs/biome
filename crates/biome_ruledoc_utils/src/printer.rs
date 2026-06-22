use biome_console::{Console, markup};
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_json_syntax::TextSize;

pub trait DiagnosticWriter {
    fn write_diagnostic(&mut self, diag: biome_diagnostics::Error);
    fn write_parse_error(&mut self, diag: biome_diagnostics::Error);
    fn print_all_diagnostics(&mut self);
    fn subtract_offset(&mut self, offset: TextSize);
}

#[derive(Default)]
pub struct DiagnosticConsoleWriter {
    pub all_diagnostics: Vec<biome_diagnostics::Error>,
    pub has_parse_error: bool,
    pub subtract_offset: TextSize,
}

impl DiagnosticWriter for DiagnosticConsoleWriter {
    fn write_diagnostic(&mut self, diag: biome_diagnostics::Error) {
        self.all_diagnostics.push(self.adjust_span_offset(diag));
    }

    fn write_parse_error(&mut self, diag: biome_diagnostics::Error) {
        self.has_parse_error = true;
        self.write_diagnostic(diag);
    }

    /// Prints all diagnostics to help the user.
    fn print_all_diagnostics(&mut self) {
        let mut console = biome_console::EnvConsole::default();
        for diag in self.all_diagnostics.iter() {
            console.println(
                biome_console::LogLevel::Error,
                markup! {
                    {PrintDiagnostic::verbose(diag)}
                },
            );
        }
    }

    fn subtract_offset(&mut self, offset: TextSize) {
        self.subtract_offset = offset;
    }
}

impl DiagnosticConsoleWriter {
    /// Adjusts the location of the diagnostic to account for synthetic nodes
    /// that arent't present in the source code but only in the AST.
    pub fn adjust_span_offset(&self, diag: biome_diagnostics::Error) -> biome_diagnostics::Error {
        if self.subtract_offset != 0.into() {
            if let Some(span) = diag.location().span {
                let new_span = span.checked_sub(self.subtract_offset);
                diag.with_file_span(new_span)
            } else {
                diag
            }
        } else {
            diag
        }
    }
}
