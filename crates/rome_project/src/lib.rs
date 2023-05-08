mod diagnostics;
mod license;
mod node_js_project;

pub use crate::diagnostics::{ProjectAnalyzeDiagnostic, ProjectDiagnostic};
pub use license::generated::*;
pub use node_js_project::{NodeJsProject, PackageJson};
use rome_deserialize::{DeserializationDiagnostic, Deserialized};
use rome_diagnostics::serde::Diagnostic;
use rome_parser::diagnostic::ParseDiagnostic;
use rome_rowan::{Language, SyntaxNode};
use std::any::TypeId;
use std::fmt::Debug;
use std::path::Path;

pub trait Manifest: Default + Debug {
    type Language: Language;

    /// It loads the manifest of the project. It accepts the path where the manifest should be
    fn deserialize_manifest(content: &SyntaxNode<Self::Language>) -> Deserialized<Self>;
}

/// An internal representation of a project.
pub trait Project {
    type Manifest: Manifest;

    /// Use this function to prepare the project, like loading the manifest.
    fn deserialize_manifest(
        &mut self,
        root: &SyntaxNode<<<Self as Project>::Manifest as Manifest>::Language>,
    );

    /// The home directory of the project
    fn project_path(&self) -> &Path;

    fn manifest(&self) -> Option<&Self::Manifest> {
        None
    }

    fn analyze(&self) -> Result<ProjectAnalyzeResult, ProjectDiagnostic>;
}

pub struct ProjectAnalyzeResult {
    _diagnostics: Vec<ProjectAnalyzeDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct AnyProject {
    pub project_type: TypeId,
    pub parse_diagnostics: Vec<ParseDiagnostic>,
    pub deserialize_diagnostics: Vec<DeserializationDiagnostic>,
}

impl AnyProject {
    pub fn new(
        project_type: TypeId,
        deserialize_diagnostics: Vec<DeserializationDiagnostic>,
        parse_diagnostics: Vec<ParseDiagnostic>,
    ) -> Self {
        Self {
            project_type,
            deserialize_diagnostics,
            parse_diagnostics,
        }
    }

    pub fn into_serde_diagnostics(self) -> Vec<Diagnostic> {
        self.parse_diagnostics
            .into_iter()
            .map(Diagnostic::new)
            .chain(
                self.deserialize_diagnostics
                    .into_iter()
                    .map(Diagnostic::new),
            )
            .collect()
    }
}

//
// impl AnyProject {
// 	pub fn try_from<T>(&self, manifest_name: &str) -> Result<T, ProjectDiagnostic> {
// 		let project_type = self.type_id();
// 		if project_type == self.project_type {
// 			match manifest_name { }
// 		} else {
// 			Err(ProjectDiagnostic::new_internal())
// 		}
// 	}
//
// }
