mod package_json;

use crate::diagnostics::ProjectDiagnostic;
pub use crate::node_js_project::package_json::PackageJson;
use crate::{Manifest, Project, ProjectAnalyzeResult};
use rome_diagnostics::Error;
use rome_json_syntax::{JsonRoot, JsonSyntaxNode};
use rome_rowan::AstNode;
use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
/// A Node.js project.
pub struct NodeJsProject {
    /// The path where the project
    manifest_path: PathBuf,
    /// The `package.json` manifest
    manifest: PackageJson,
    /// Diagnostics emitted during the operations
    pub diagnostics: Vec<Error>,
}

impl NodeJsProject {
    pub fn deserialize(&mut self, root: JsonRoot) {
        self.deserialize_manifest(root.syntax());
    }
}

impl Project for NodeJsProject {
    type Manifest = PackageJson;

    fn deserialize_manifest(&mut self, content: &JsonSyntaxNode) {
        let manifest = Self::Manifest::deserialize_manifest(content);
        let (package, deserialize_diagnostics) = manifest.consume();
        self.manifest = package;
        self.diagnostics = deserialize_diagnostics;
    }

    fn project_path(&self) -> &Path {
        self.manifest_path.as_path()
    }

    fn manifest(&self) -> Option<&Self::Manifest> {
        Some(&self.manifest)
    }

    fn analyze(&self) -> Result<ProjectAnalyzeResult, ProjectDiagnostic> {
        Ok(ProjectAnalyzeResult {
            _diagnostics: vec![],
        })
    }
}
