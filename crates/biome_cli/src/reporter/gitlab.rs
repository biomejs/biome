use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::{
    display_gitlab::{GitLabDiagnostic, PrintGitLabDiagnostic},
    Error, Resource,
};
use path_absolutize::Absolutize;
use std::sync::RwLock;
use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

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
    repository_root: Option<PathBuf>,
}

#[derive(Default)]
struct GitlabHasher(HashSet<u64>);

impl GitlabHasher {
    /// Enforces uniqueness of generated fingerprints in the context of a
    /// single report.
    fn rehash_until_unique(&mut self, fingerprint: u64) -> u64 {
        let mut current = fingerprint;
        while self.0.contains(&current) {
            let mut hasher = DefaultHasher::new();
            current.hash(&mut hasher);
            current = hasher.finish();
        }

        self.0.insert(current);
        current
    }
}

impl<'a> GitLabReporterVisitor<'a> {
    pub fn new(console: &'a mut dyn Console, repository_root: Option<PathBuf>) -> Self {
        Self {
            console,
            repository_root,
        }
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
        let hasher = RwLock::default();
        let diagnostics = GitlabDiagnostics(payload, &hasher, self.repository_root.as_deref());
        self.console.log(markup!({ diagnostics }));
        Ok(())
    }
}

struct GitlabDiagnostics<'a>(
    DiagnosticsPayload,
    &'a RwLock<GitlabHasher>,
    Option<&'a Path>,
);

impl<'a> GitlabDiagnostics<'a> {
    fn attempt_to_relativize(&self, subject: &str) -> Option<PathBuf> {
        let Some(base) = self.2 else {
            return None;
        };

        let Ok(resolved) = Path::new(subject).absolutize() else {
            return None;
        };

        let Ok(relativized) = resolved.strip_prefix(base) else {
            return None;
        };

        Some(relativized.to_path_buf())
    }

    fn compute_initial_fingerprint(&self, diagnostic: &Error, path: &str) -> u64 {
        let location = diagnostic.location();
        let code = match location.span {
            Some(span) => match location.source_code {
                Some(source_code) => &source_code.text[span],
                None => "",
            },
            None => "",
        };

        let check_name = diagnostic
            .category()
            .map(|category| category.name())
            .unwrap_or_default();

        calculate_hash(&Fingerprint {
            check_name,
            path,
            code,
        })
    }
}

impl<'a> Display for GitlabDiagnostics<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_str("[")?;

        let mut hasher = self.1.write().unwrap();

        let mut iter = self
            .0
            .diagnostics
            .iter()
            .filter(|d| d.severity() >= self.0.diagnostic_level)
            .filter(|d| {
                if self.0.verbose {
                    if d.tags().is_verbose() {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .peekable();
        while let Some(biome_diagnostic) = iter.next() {
            if biome_diagnostic.severity() >= self.0.diagnostic_level {
                let absolute_path = match biome_diagnostic.location().resource {
                    Some(Resource::File(file)) => Some(file),
                    _ => None,
                }
                .unwrap_or_default();
                let path_buf = self.attempt_to_relativize(absolute_path);
                let path = match path_buf {
                    Some(buf) => buf.to_str().unwrap_or(absolute_path).to_owned(),
                    None => absolute_path.to_owned(),
                };

                let initial_fingerprint =
                    self.compute_initial_fingerprint(&biome_diagnostic, &path);
                let fingerprint = hasher.rehash_until_unique(initial_fingerprint);

                let Some(gitlab_diagnostic) =
                    GitLabDiagnostic::try_from_diagnostic(biome_diagnostic, &path, fingerprint)
                else {
                    continue;
                };

                fmt.write_markup(markup!({
                    PrintGitLabDiagnostic {
                        gitlab_diagnostic: &gitlab_diagnostic,
                    }
                }))?;

                if !iter.peek().is_none() {
                    fmt.write_str(",")?;
                }
            }
        }
        fmt.write_str("]")?;
        Ok(())
    }
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
