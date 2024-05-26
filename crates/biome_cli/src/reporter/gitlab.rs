use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::Formatter;
use biome_diagnostics::{LineIndexBuf, PrintDescription};
use serde::Serialize;
use std::{
    cmp::max,
    collections::HashSet,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

/// An entry in the GitLab Code Quality report.
/// See https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool
#[derive(Serialize)]
pub(crate) struct GitLabDiagnostic {
    /// A description of the code quality violation.
    description: String,
    /// A unique name representing the static analysis check that emitted this issue.
    check_name: String,
    /// A unique fingerprint to identify the code quality violation. For example, an MD5 hash.
    fingerprint: String,
    /// A severity string (can be info, minor, major, critical, or blocker).
    severity: GitLabSeverity,
    /// The location where the code quality violation occurred.
    location: Location,
}

#[derive(Serialize)]
struct Lines {
    /// The line on which the code quality violation occurred.
    begin: u8,
}

#[derive(Serialize)]
struct Location {
    /// The relative path to the file containing the code quality violation.
    path: String,
    lines: Lines,
}

#[derive(Serialize)]
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

pub(crate) struct GitLabReporterVisitor {
    diagnostics: Vec<GitLabDiagnostic>,
    builder: GitLabDiagnosticBuilder,
}

impl GitLabReporterVisitor {
    pub fn new(builder: GitLabDiagnosticBuilder) -> Self {
        Self {
            diagnostics: Vec::new(),
            builder,
        }
    }
}

impl biome_console::fmt::Display for GitLabReporterVisitor {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let content = serde_json::to_string(&self.diagnostics)?;
        fmt.write_str(content.as_str())
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
        for biome_diagnostic in payload.diagnostics {
            if biome_diagnostic.severity() >= payload.diagnostic_level {
                if biome_diagnostic.tags().is_verbose() {
                    if payload.verbose {
                        self.diagnostics
                            .push(self.builder.gitlab_diagnostic(biome_diagnostic))
                    }
                } else {
                    self.diagnostics
                        .push(self.builder.gitlab_diagnostic(biome_diagnostic))
                }
            }
        }
        Ok(())
    }
}

pub(crate) struct GitLabDiagnosticBuilder {
    repository_root: Option<PathBuf>,
    fingerprints: HashSet<u64>,
}

impl GitLabDiagnosticBuilder {
    pub fn new(repository_root: Option<PathBuf>) -> Self {
        Self {
            repository_root,
            fingerprints: HashSet::new(),
        }
    }

    pub fn gitlab_diagnostic(
        &mut self,
        value: biome_diagnostics::error::Error,
    ) -> GitLabDiagnostic {
        let check_name = value.category().map_or("unknown".to_string(), |category| {
            category.name().to_string()
        });
        let path = self.path(&value).unwrap_or("unknown".to_string());
        let line = self.line(&value).map_or(1, |line| max(line, 1));
        let fingerprint = self
            .ensure_fingerprint_uniqueness(self.fingerprint(&check_name, &path, line), 0)
            .to_string();

        GitLabDiagnostic {
            severity: GitLabSeverity::from(value.severity()),
            description: PrintDescription(&value).to_string(),
            check_name,
            fingerprint,
            location: Location {
                path,
                lines: Lines { begin: line },
            },
        }
    }

    fn fingerprint(&self, check_name: &String, path: &String, line: u8) -> u64 {
        let mut hasher = DefaultHasher::new();

        check_name.hash(&mut hasher);
        path.hash(&mut hasher);
        line.hash(&mut hasher);

        hasher.finish()
    }

    fn ensure_fingerprint_uniqueness(&mut self, fingerprint: u64, salt: u64) -> u64 {
        let mut current = fingerprint;
        while self.fingerprints.contains(&current) {
            let mut hasher = DefaultHasher::new();
            current.hash(&mut hasher);
            salt.hash(&mut hasher);
            current = hasher.finish();
        }

        self.fingerprints.insert(current);
        current
    }

    fn path(&self, value: &biome_diagnostics::error::Error) -> Option<String> {
        let path = match value.location().resource? {
            biome_diagnostics::Resource::Argv => None,
            biome_diagnostics::Resource::Memory => None,
            biome_diagnostics::Resource::File(path) => Some(path.to_string()),
        }?;

        let Some(root) = &self.repository_root else {
            return Some(path);
        };

        let Ok(relativized) = Path::new(path.as_str()).strip_prefix(root) else {
            return Some(path);
        };

        Some(relativized.to_str().unwrap_or(path.as_str()).to_string())
    }

    fn line(&self, value: &biome_diagnostics::error::Error) -> Option<u8> {
        let location = value.location();
        let buf = LineIndexBuf::from_source_text(location.source_code?.text);
        let diagnostic_offset = location.span?.start();

        buf.iter()
            .enumerate()
            .find(|(_, line_offset)| **line_offset >= diagnostic_offset)
            .map(|(line_number, _)| u8::try_from(line_number).unwrap())
    }
}
