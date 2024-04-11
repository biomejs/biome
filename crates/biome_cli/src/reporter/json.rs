use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::Formatter;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JsonReporterVisitor {
    summary: TraversalSummary,
    diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    command: String,
}

impl JsonReporterVisitor {
    pub(crate) fn new(summary: TraversalSummary) -> Self {
        Self {
            summary,
            diagnostics: vec![],
            command: String::new(),
        }
    }
}

impl biome_console::fmt::Display for JsonReporterVisitor {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let content = serde_json::to_string(&self)?;
        fmt.write_str(content.as_str())
    }
}

pub struct JsonReporter {
    pub execution: Execution,
    pub diagnostics: DiagnosticsPayload,
    pub summary: TraversalSummary,
}

impl Reporter for JsonReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_summary(&self.execution, self.summary)?;
        visitor.report_diagnostics(&self.execution, self.diagnostics)?;

        Ok(())
    }
}

impl ReporterVisitor for JsonReporterVisitor {
    fn report_summary(
        &mut self,
        execution: &Execution,
        summary: TraversalSummary,
    ) -> std::io::Result<()> {
        self.summary = summary;
        self.command = format!("{}", execution.traversal_mode());

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        payload: DiagnosticsPayload,
    ) -> std::io::Result<()> {
        for diagnostic in payload.diagnostics {
            if diagnostic.severity() >= payload.diagnostic_level {
                if diagnostic.tags().is_verbose() {
                    if payload.verbose {
                        self.diagnostics
                            .push(biome_diagnostics::serde::Diagnostic::new(diagnostic))
                    }
                } else {
                    self.diagnostics
                        .push(biome_diagnostics::serde::Diagnostic::new(diagnostic))
                }
            }
        }
        Ok(())
    }
}
