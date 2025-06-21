use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::{Error, Resource, Severity};
use std::collections::BTreeMap;
use std::io::{self, Write};

pub struct CheckstyleReporter {
    pub summary: TraversalSummary,
    pub diagnostics_payload: DiagnosticsPayload,
    pub execution: Execution,
}

impl Reporter for CheckstyleReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_summary(&self.execution, self.summary)?;
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload)?;
        Ok(())
    }
}

pub struct CheckstyleReporterVisitor<'a> {
    console: &'a mut dyn Console,
}

impl<'a> CheckstyleReporterVisitor<'a> {
    pub fn new(console: &'a mut dyn Console) -> Self {
        Self { console }
    }
}

impl<'a> ReporterVisitor for CheckstyleReporterVisitor<'a> {
    fn report_summary(&mut self, _execution: &Execution, _summary: TraversalSummary) -> io::Result<()> {
        // Checkstyle does not require a summary
        Ok(())
    }

    fn report_diagnostics(&mut self, _execution: &Execution, payload: DiagnosticsPayload) -> io::Result<()> {
        let mut files: BTreeMap<String, Vec<&Error>> = BTreeMap::new();
        for diagnostic in &payload.diagnostics {
            if diagnostic.severity() >= payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && !payload.verbose {
                    continue;
                }
                let path = match diagnostic.location().resource {
                    Some(Resource::File(file)) => file.to_string(),
                    _ => "<unknown>".to_string(),
                };
                files.entry(path).or_default().push(diagnostic);
            }
        }
        let mut output = Vec::new();
        writeln!(output, "<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
        writeln!(output, "<checkstyle version=\"4.3\">")?;
        for (file, diagnostics) in files {
            writeln!(output, "  <file name=\"{}\">", xml_escape(&file))?;
            for diagnostic in diagnostics {
                let location = diagnostic.location();
                let (line, column) = if let (Some(span), Some(source_code)) = (location.span, location.source_code) {
                    let source = biome_diagnostics::display::SourceFile::new(source_code);
                    if let Ok(start) = source.location(span.start()) {
                        (start.line_number.get(), start.column_number.get())
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                };
                let severity = match diagnostic.severity() {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                    Severity::Information | Severity::Hint => "info",
                    Severity::Fatal => "error",
                };
                let message = get_error_description(diagnostic);
                let source = diagnostic.category().map(|c| c.name()).unwrap_or("");
                writeln!(output,
                    "    <error line=\"{}\" column=\"{}\" severity=\"{}\" message=\"{}\" source=\"{}\" />",
                    line,
                    column,
                    severity,
                    xml_escape(&message),
                    xml_escape(source),
                )?;
            }
            writeln!(output, "  </file>")?;
        }
        writeln!(output, "</checkstyle>")?;
        self.console.log(markup! {{
            (String::from_utf8_lossy(&output))
        }});
        Ok(())
    }
}

fn xml_escape(input: &str) -> String {
    input.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn get_error_description(error: &Error) -> String {
    format!("{:?}", error)
} 