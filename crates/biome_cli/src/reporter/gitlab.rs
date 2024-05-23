use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::Formatter;
use biome_diagnostics::{LineIndexBuf, PrintDescription};
use serde::Serialize;
use std::{cmp::max, fmt::Display};

#[derive(Serialize)]
struct Lines {
    begin: u8,
}

#[derive(Serialize)]
struct Location {
    path: String,
    lines: Lines,
}

enum GitLabSeverity {
    Info,
    Minor,
    Major,
    Critical,
    Blocker,
}

impl Display for GitLabSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GitLabSeverity::Info => "info".to_string(),
                GitLabSeverity::Minor => "minor".to_string(),
                GitLabSeverity::Major => "major".to_string(),
                GitLabSeverity::Critical => "critical".to_string(),
                GitLabSeverity::Blocker => "blocker".to_string(),
            }
        )
    }
}

impl From<biome_diagnostics::diagnostic::Severity> for GitLabSeverity {
    fn from(value: biome_diagnostics::diagnostic::Severity) -> Self {
        match value {
            biome_diagnostics::Severity::Hint => Self::Info,
            biome_diagnostics::Severity::Information => Self::Minor,
            biome_diagnostics::Severity::Warning => Self::Major,
            biome_diagnostics::Severity::Error => Self::Critical,
            biome_diagnostics::Severity::Fatal => Self::Blocker,
        }
    }
}

#[derive(Serialize)]
struct GitLabDiagnostic {
    description: String,
    severity: String,
    fingerprint: String,
    location: Location,
}

fn get_location_path(value: &biome_diagnostics::error::Error) -> Option<String> {
    match value.location().resource? {
        biome_diagnostics::Resource::Argv => None,
        biome_diagnostics::Resource::Memory => None,
        biome_diagnostics::Resource::File(path) => Some(path.to_string()),
    }
}

fn get_line_begin(value: &biome_diagnostics::error::Error) -> Option<u8> {
    let location = value.location();
    let buf = LineIndexBuf::from_source_text(location.source_code?.text);
    let diagnostic_offset = location.span?.start();

    buf.iter()
        .enumerate()
        .find(|(_, line_offset)| **line_offset >= diagnostic_offset)
        .map(|(line_number, _)| u8::try_from(line_number).unwrap())
}

impl From<biome_diagnostics::error::Error> for GitLabDiagnostic {
    fn from(value: biome_diagnostics::error::Error) -> Self {
        Self {
            severity: GitLabSeverity::from(value.severity()).to_string(),
            // TODO: Somehow include advices in here.
            description: PrintDescription(&value).to_string(),
            // TODO:
            fingerprint: "TODO".to_string(),
            location: Location {
                // TODO: Relativize this path
                path: get_location_path(&value).unwrap_or("unknown".to_string()),
                lines: Lines {
                    begin: get_line_begin(&value).map(|line| max(line, 1)).unwrap_or(1),
                },
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct GitLabReporterVisitor {
    diagnostics: Vec<GitLabDiagnostic>,
}

impl biome_console::fmt::Display for GitLabReporterVisitor {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let content = serde_json::to_string(&self.diagnostics)?;
        fmt.write_str(content.as_str())
    }
}

pub struct GitLabReporter {
    pub execution: Execution,
    pub diagnostics: DiagnosticsPayload,
    pub summary: TraversalSummary,
}

impl Reporter for GitLabReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_summary(&self.execution, self.summary)?;
        visitor.report_diagnostics(&self.execution, self.diagnostics)?;

        Ok(())
    }
}

impl ReporterVisitor for GitLabReporterVisitor {
    fn report_summary(&mut self, _: &Execution, _: TraversalSummary) -> std::io::Result<()> {
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
                        self.diagnostics.push(GitLabDiagnostic::from(diagnostic))
                    }
                } else {
                    self.diagnostics.push(GitLabDiagnostic::from(diagnostic))
                }
            }
        }
        Ok(())
    }
}
