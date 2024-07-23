use crate::display::frame::SourceFile;
use crate::PrintDescription;
use crate::{diagnostic::internal::AsDiagnostic, Diagnostic, Resource, Severity};
use biome_console::fmt;
use path_absolutize::Absolutize;
use serde::Serialize;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [biome_console::fmt::Write].
pub struct PrintGitLabDiagnostic<'fmt> {
    pub gitlab_diagnostic: &'fmt GitLabDiagnostic<'fmt>,

    /// Whether this is the last diagnostic to report in the run.
    pub is_last: bool,
}

impl fmt::Display for PrintGitLabDiagnostic<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let mut serialized = serde_json::to_string(self.gitlab_diagnostic)?;
        if !self.is_last {
            serialized.push(',')
        }
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
    pub fingerprint: u64,
    /// A severity string (can be info, minor, major, critical, or blocker).
    severity: &'a str,
    /// The location where the code quality violation occurred.
    location: Location,
}

impl<'a> GitLabDiagnostic<'a> {
    pub fn try_from_diagnostic<D: AsDiagnostic>(
        diag: D,
        relative_to: &Option<PathBuf>,
    ) -> Option<Self> {
        let diagnostic = diag.as_diagnostic();
        let location = diagnostic.location();

        let span = location.span?;
        let source_code = location.source_code?;

        let absolute_file_path = match &location.resource {
            Some(Resource::File(file)) => *file,
            _ => return None,
        };

        let relative_path = attempt_to_relativize(absolute_file_path, relative_to)
            .unwrap_or(absolute_file_path.to_string());

        let description = PrintDescription(&diag).to_string();

        let begin = match SourceFile::new(source_code).location(span.start()) {
            Ok(start) => start.line_number.get(),
            Err(_) => return None,
        };

        let check_name = diagnostic
            .category()
            .map(|category| category.name())
            .unwrap_or_default();

        let fingerprint = calculate_hash(&Fingerprint {
            check_name,
            path: relative_path.as_str(),
            code: &source_code.text[span],
        });

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
            fingerprint,
            location: Location {
                path: relative_path,
                lines: Lines { begin },
            },
        })
    }
}

fn attempt_to_relativize(subject: &str, maybe_base: &Option<PathBuf>) -> Option<String> {
    let Some(base) = maybe_base else {
        return None;
    };

    let Ok(resolved) = Path::new(subject).absolutize() else {
        return None;
    };

    let Ok(relativized) = resolved.strip_prefix(base) else {
        return None;
    };

    Some(relativized.to_path_buf().to_str()?.to_string())
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
    begin: usize,
}

#[derive(Hash)]
struct Fingerprint<'a> {
    // Including the source code in our hash leads to more stable
    // fingerprints. If you instead rely on e.g. the line number and change
    // the first line of a file, all of its fingerprint would change.
    code: &'a str,
    check_name: &'a str,
    path: &'a str,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
