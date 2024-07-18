use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::{LineIndexBuf, PrintDescription};
use path_absolutize::Absolutize;
use serde::Serialize;
use std::{
    cmp::max,
    collections::HashSet,
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
struct Location {
    /// The relative path to the file containing the code quality violation.
    path: String,
    lines: Lines,
}

#[derive(Serialize)]
struct Lines {
    /// The line on which the code quality violation occurred.
    begin: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum GitLabSeverity {
    Info,
    Minor,
    Major,
    Critical,
    Blocker,
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
}

impl Reporter for GitLabReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_diagnostics(&self.execution, self.diagnostics)?;
        Ok(())
    }
}

pub(crate) struct GitLabReporterVisitor<'a> {
    console: &'a mut dyn Console,
    builder: GitLabDiagnosticBuilder,
}

impl<'a> GitLabReporterVisitor<'a> {
    pub fn new(builder: GitLabDiagnosticBuilder, console: &'a mut dyn Console) -> Self {
        Self { builder, console }
    }
}

impl<'a> ReporterVisitor for GitLabReporterVisitor<'a> {
    fn report_summary(&mut self, _: &Execution, _: TraversalSummary) -> std::io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        payload: DiagnosticsPayload,
    ) -> std::io::Result<()> {
        self.console.log(markup!("["));

        let total_diagnostics = payload.diagnostics.len();
        for (index, biome_diagnostic) in payload.diagnostics.into_iter().enumerate() {
            if biome_diagnostic.severity() >= payload.diagnostic_level {
                if biome_diagnostic.tags().is_verbose() && !payload.verbose {
                    continue;
                }

                let diagnostic = self.builder.gitlab_diagnostic(biome_diagnostic);
                let mut content = serde_json::to_string(&diagnostic)?;
                if index < total_diagnostics - 1 {
                    content.push(',')
                }

                self.console.log(markup!({ content }));
            }
        }
        self.console.log(markup!("]"));
        Ok(())
    }
}

pub(crate) struct GitLabDiagnosticBuilder {
    /// The root of the Git repository.
    /// Required for relativization of the reported absolute paths.
    repository_root: Option<PathBuf>,
    /// A set of fingerprints (unique identifiers) to prevent collisions.
    fingerprints: HashSet<u64>,
}

impl GitLabDiagnosticBuilder {
    pub fn new(repository_root: Option<PathBuf>) -> Self {
        Self {
            repository_root,
            fingerprints: HashSet::new(),
        }
    }

    /// Turns a biome diagnostic into one in the GitLab Code Quality report format.
    pub fn gitlab_diagnostic(
        &mut self,
        value: biome_diagnostics::error::Error,
    ) -> GitLabDiagnostic {
        let check_name = self.check_name(&value).unwrap_or_default();
        let path = self.path(&value).unwrap_or_default();
        let line = self.line(&value).map_or(1, |line| max(line, 1));
        let source_code = self.source_code(&value).unwrap_or_default();
        let fingerprint = self
            .ensure_fingerprint_uniqueness(self.fingerprint(&check_name, &path, &source_code), 0)
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

    /// Extracts the name of the category as the check name.
    fn check_name(&self, value: &biome_diagnostics::error::Error) -> Option<String> {
        let category = value.category()?;
        Some(category.name().to_string())
    }

    /// Extracts the source code generating the diagnostic.
    fn source_code(&self, value: &biome_diagnostics::error::Error) -> Option<String> {
        let location = value.location();
        let source_code = location.source_code?;
        let span = location.span?;

        Some(source_code.text[span].to_string())
    }

    /// Generates a fingerprint for a diagnostic.
    fn fingerprint(&self, check_name: &String, path: &String, code: &String) -> u64 {
        let mut hasher = DefaultHasher::new();

        // Including the source code in our hash leads to more stable
        // fingerprints. If you instead rely on e.g. the line number and change
        // the first line of a file, all of its fingerprint would change.
        code.hash(&mut hasher);
        check_name.hash(&mut hasher);
        path.hash(&mut hasher);

        hasher.finish()
    }

    /// Enforces uniqueness of generated fingerprints in the context of a
    /// single report.
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

    /// GitLab only cares about paths relative to the repository root.
    /// This function attempts to relativize the absolute path from the
    /// diagnostic, while falling back to the originally reported one.
    fn path(&self, value: &biome_diagnostics::error::Error) -> Option<String> {
        let path = match value.location().resource? {
            biome_diagnostics::Resource::Argv => None,
            biome_diagnostics::Resource::Memory => None,
            biome_diagnostics::Resource::File(path) => Some(path.to_string()),
        }?;

        let Some(root) = &self.repository_root else {
            return Some(path);
        };

        let Ok(resolved) = Path::new(path.as_str()).absolutize() else {
            return Some(path);
        };

        let Ok(relativized) = resolved.strip_prefix(root) else {
            return Some(path);
        };

        Some(relativized.to_str().unwrap_or(path.as_str()).to_string())
    }

    /// Extracts the line number from the diagnostic.
    fn line(&self, value: &biome_diagnostics::error::Error) -> Option<u32> {
        let location = value.location();
        let buf = LineIndexBuf::from_source_text(location.source_code?.text);
        let diagnostic_offset = location.span?.start();

        buf.iter()
            .enumerate()
            .find(|(_, line_offset)| **line_offset >= diagnostic_offset)
            .map(|(line_number, _)| u32::try_from(line_number).unwrap())
    }
}
