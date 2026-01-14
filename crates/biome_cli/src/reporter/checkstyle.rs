use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::markup;
use biome_diagnostics::display::SourceFile;
use biome_diagnostics::{Error, PrintDescription, Resource, Severity};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeMap;
use std::io::{self, Write};

pub struct CheckstyleReporter<'a> {
    pub summary: TraversalSummary,
    pub diagnostics_payload: &'a DiagnosticsPayload,
    pub execution: &'a dyn Execution,
    pub verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for CheckstyleReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()> {
        visitor.report_summary(writer, self.execution, self.summary, self.verbose)?;
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

pub struct CheckstyleReporterVisitor;

impl ReporterVisitor for CheckstyleReporterVisitor {
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
        payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        let mut files: BTreeMap<String, Vec<&Error>> = BTreeMap::new();
        for diagnostic in &payload.diagnostics {
            if diagnostic.severity() >= payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && !verbose {
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
                let (line, column) = if let (Some(span), Some(source_code)) =
                    (location.span, location.source_code)
                {
                    let source = SourceFile::new(source_code);
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
                let description = PrintDescription(diagnostic).to_string();
                let source = diagnostic.category().map_or("", |c| c.name());
                writeln!(
                    output,
                    "    <error line=\"{}\" column=\"{}\" severity=\"{}\" message=\"{}\" source=\"{}\" />",
                    line,
                    column,
                    severity,
                    xml_escape(&description),
                    xml_escape(source),
                )?;
            }
            writeln!(output, "  </file>")?;
        }
        writeln!(output, "</checkstyle>")?;
        writer.log(markup! {{
            (String::from_utf8_lossy(&output))
        }});
        Ok(())
    }
}

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
