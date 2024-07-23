use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::display_gitlab::{GitLabDiagnostic, PrintGitLabDiagnostic};
use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
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

    /// A set of fingerprints (unique identifiers) to prevent collisions.
    fingerprints: HashSet<u64>,
}

impl<'a> GitLabReporterVisitor<'a> {
    pub fn new(console: &'a mut dyn Console, repository_root: Option<PathBuf>) -> Self {
        Self {
            console,
            repository_root,
            fingerprints: HashSet::new(),
        }
    }

    /// Enforces uniqueness of generated fingerprints in the context of a
    /// single report.
    fn rehash_until_unique(&mut self, fingerprint: u64) -> u64 {
        let mut current = fingerprint;
        while self.fingerprints.contains(&current) {
            let mut hasher = DefaultHasher::new();
            current.hash(&mut hasher);
            current = hasher.finish();
        }

        self.fingerprints.insert(current);
        current
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

                let Some(mut gitlab_diagnostic) =
                    GitLabDiagnostic::try_from_diagnostic(biome_diagnostic, &self.repository_root)
                else {
                    continue;
                };

                gitlab_diagnostic.fingerprint =
                    self.rehash_until_unique(gitlab_diagnostic.fingerprint);

                self.console.log(markup!({
                    PrintGitLabDiagnostic {
                        gitlab_diagnostic: &gitlab_diagnostic,
                        is_last: index < total_diagnostics - 1,
                    }
                }));
            }
        }
        self.console.log(markup!("]"));
        Ok(())
    }
}
