mod diagnostics;
mod license;
mod node_js_package;

pub use crate::diagnostics::{ProjectAnalyzeDiagnostic, ProjectDiagnostic};
pub use license::generated::*;
pub use node_js_package::{
    Dependencies, NodeJsPackage, PackageJson, PackageType, TsConfigJson, Version,
};

use std::any::TypeId;
use std::fmt::Debug;

use biome_deserialize::{DeserializationDiagnostic, Deserialized};
use biome_diagnostics::serde::Diagnostic;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::Language;

pub(crate) type LanguageRoot<L> = <L as Language>::Root;

pub(crate) type PackageRoot<P> =
    <<<P as Package>::Manifest as Manifest>::Language as Language>::Root;

pub trait Manifest: Debug + Sized {
    type Language: Language;

    /// It loads the manifest of the package. It accepts the path where the manifest should be
    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self>;
}

/// An internal representation of a package.
pub trait Package {
    type Manifest: Manifest;

    /// Inserts a manifest into the package, taking care of deserialization.
    fn insert_serialized_manifest(&mut self, root: &PackageRoot<Self>);

    fn manifest(&self) -> Option<&Self::Manifest> {
        None
    }

    fn analyze(&self) -> PackageAnalyzeResult;

    fn has_errors(&self) -> bool;
}

pub struct PackageAnalyzeResult {
    pub diagnostics: Vec<ProjectAnalyzeDiagnostic>,
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
