use crate::display::frame::SourceFile;
use crate::PrintDescription;
use crate::{diagnostic::internal::AsDiagnostic, Diagnostic, Severity};
use biome_console::fmt;
use serde::Serialize;
use std::io;

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [biome_console::fmt::Write].
pub struct PrintGitLabDiagnostic<'fmt> {
    pub gitlab_diagnostic: &'fmt GitLabDiagnostic<'fmt>,
}

impl fmt::Display for PrintGitLabDiagnostic<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(self.gitlab_diagnostic)?;
        fmt.write_str(serialized.as_str())?;

        Ok(())
    }
}

/// An entry in the GitLab Code Quality report.
/// See https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool
#[derive(Serialize)]
pub struct GitLabDiagnostic<'a> {
    /// A description of the code quality violation.
    description: String,
    /// A unique name representing the static analysis check that emitted this issue.
    check_name: &'a str,
    /// A unique fingerprint to identify the code quality violation. For example, an MD5 hash.
    fingerprint: String,
    /// A severity string (can be info, minor, major, critical, or blocker).
    severity: &'a str,
    /// The location where the code quality violation occurred.
    location: Location<'a>,
}

impl<'a> GitLabDiagnostic<'a> {
    pub fn try_from_diagnostic<D: AsDiagnostic>(
        diag: &'a D,
        path: &'a str,
        fingerprint: u64,
    ) -> Option<Self> {
        let diagnostic = diag.as_diagnostic();
        let location = diagnostic.location();
        let span = location.span?;
        let source_code = location.source_code?;
        let description = PrintDescription(diag).to_string();
        let begin = match SourceFile::new(source_code).location(span.start()) {
            Ok(start) => start.line_number.get(),
            Err(_) => return None,
        };
        let check_name = diagnostic
            .category()
            .map(|category| category.name())
            .unwrap_or_default();

        Some(GitLabDiagnostic {
            severity: match diagnostic.severity() {
                Severity::Hint => "info",
                Severity::Information => "minor",
                Severity::Warning => "major",
                Severity::Error => "critical",
                Severity::Fatal => "blocker",
            },
            description,
            check_name,
            // A u64 does not fit into a JSON number, so we serialize this as a
            // string
            fingerprint: fingerprint.to_string(),
            location: Location {
                path,
                lines: Lines { begin },
            },
        })
    }
}

#[derive(Serialize)]
struct Location<'a> {
    /// The relative path to the file containing the code quality violation.
    path: &'a str,
    lines: Lines,
}

#[derive(Serialize)]
struct Lines {
    /// The line on which the code quality violation occurred.
    begin: usize,
}
