use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::Formatter;
use biome_diagnostics::{Diagnostic, DiagnosticExt};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FilesReporterVisitor {
    summary: TraversalSummary,
    files: Vec<String>,
    command: String,
}

impl FilesReporterVisitor {
    pub(crate) fn new(summary: TraversalSummary) -> Self {
        Self {
            summary,
            files: vec![],
            command: String::new(),
        }
    }
}

impl biome_console::fmt::Display for FilesReporterVisitor {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let content = serde_json::to_string(&self)?;
        fmt.write_str(content.as_str())
    }
}

pub struct FilesReporter {
    pub execution: Execution,
    pub diagnostics: DiagnosticsPayload,
    pub summary: TraversalSummary,
}

impl Reporter for FilesReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_summary(&self.execution, self.summary)?;
        visitor.report_diagnostics(&self.execution, self.diagnostics)?;

        Ok(())
    }
}

impl ReporterVisitor for FilesReporterVisitor {
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
                        // let location =
                        //     biome_diagnostics::serde::Diagnostic::new(diagnostic).location();

                        println!(
                            "{:#?}",
                            biome_diagnostics::serde::Diagnostic::new(diagnostic).location()
                        )
                        // self.files.push(location.resource)
                    }
                } else {
                    println!(
                        "{:#?}",
                        biome_diagnostics::serde::Diagnostic::new(diagnostic).location()
                    )
                    // self.diagnostics
                    // .push(biome_diagnostics::serde::Diagnostic::new(diagnostic))
                }
            }
        }
        Ok(())
    }
}
