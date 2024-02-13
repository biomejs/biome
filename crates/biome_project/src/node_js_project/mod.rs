mod package_json;

pub use crate::node_js_project::package_json::PackageJson;
use crate::{Manifest, Project, ProjectAnalyzeDiagnostic, ProjectAnalyzeResult, LICENSE_LIST};
use biome_json_syntax::JsonRoot;
use biome_rowan::Language;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone)]
/// A Node.js project.
pub struct NodeJsProject {
    /// The path where the project
    pub manifest_path: PathBuf,
    /// The `package.json` manifest
    pub manifest: PackageJson,
    /// Diagnostics emitted during the operations
    pub diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
}

impl NodeJsProject {
    pub fn from_root(&mut self, root: &JsonRoot) {
        self.deserialize_manifest(root);
    }
}

pub(crate) type ProjectLanguageRoot<M> = <<M as Manifest>::Language as Language>::Root;

impl Project for NodeJsProject {
    type Manifest = PackageJson;

    fn deserialize_manifest(&mut self, content: &ProjectLanguageRoot<Self::Manifest>) {
        let manifest = Self::Manifest::deserialize_manifest(content);
        let (package, deserialize_diagnostics) = manifest.consume();
        self.manifest = package.unwrap_or_default();
        self.diagnostics = deserialize_diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect();
    }

    fn project_path(&self) -> &Path {
        self.manifest_path.as_path()
    }

    fn manifest(&self) -> Option<&Self::Manifest> {
        Some(&self.manifest)
    }

    fn analyze(&self) -> ProjectAnalyzeResult {
        let mut diagnostics = vec![];
        if let Some((license, range)) = &self.manifest.license {
            if !LICENSE_LIST.is_valid(license) {
                diagnostics
                    .push(ProjectAnalyzeDiagnostic::new_invalid_license(license).with_range(range))
            } else if !LICENSE_LIST.is_deprecated(license) {
                diagnostics.push(
                    ProjectAnalyzeDiagnostic::new_deprecated_license(license).with_range(range),
                )
            }
        }

        ProjectAnalyzeResult { diagnostics }
    }

    fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
