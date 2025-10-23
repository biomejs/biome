mod package_json;
mod tsconfig_json;

use camino::Utf8Path;
pub use package_json::{Dependencies, PackageJson, PackageType, Version};
pub use tsconfig_json::{CompilerOptions, TsConfigJson};

use biome_rowan::Language;

use crate::{LICENSE_LIST, Manifest, Package, PackageAnalyzeResult, ProjectAnalyzeDiagnostic};

#[derive(Default, Debug, Clone)]
/// A Node.js project.
pub struct NodeJsPackage {
    /// The `package.json` manifest
    pub manifest: Option<PackageJson>,

    /// Diagnostics emitted during the operations
    pub diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,

    /// The `tsconfig.json` manifest
    pub tsconfig: Option<TsConfigJson>,
}

impl NodeJsPackage {
    pub fn insert_serialized_tsconfig(
        &mut self,
        content: &ProjectLanguageRoot<TsConfigJson>,
        path: &Utf8Path,
    ) {
        let tsconfig = TsConfigJson::deserialize_manifest(content, path);
        let (tsconfig, deserialize_diagnostics) = tsconfig.consume();
        self.tsconfig = Some(tsconfig.unwrap_or_default());
        self.diagnostics = deserialize_diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect();
    }

    pub fn without_tsconfig(&self) -> Self {
        Self {
            manifest: self.manifest.clone(),
            diagnostics: self.diagnostics.clone(),
            tsconfig: None,
        }
    }
}

pub(crate) type ProjectLanguageRoot<M> = <<M as Manifest>::Language as Language>::Root;

impl Package for NodeJsPackage {
    type Manifest = PackageJson;

    fn insert_serialized_manifest(
        &mut self,
        content: &ProjectLanguageRoot<Self::Manifest>,
        path: &Utf8Path,
    ) {
        let deserialized = Self::Manifest::deserialize_manifest(content, path);
        let (manifest, diagnostics) = deserialized.consume();
        self.manifest = manifest;
        self.diagnostics = diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect();
    }

    fn manifest(&self) -> Option<&Self::Manifest> {
        self.manifest.as_ref()
    }

    fn analyze(&self) -> PackageAnalyzeResult {
        let mut diagnostics = vec![];
        if let Some((license, range)) = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.license.as_ref())
        {
            if !LICENSE_LIST.is_valid(license) {
                diagnostics.push(
                    ProjectAnalyzeDiagnostic::new_invalid_license(license.to_string())
                        .with_range(range),
                )
            } else if LICENSE_LIST.is_deprecated(license) {
                diagnostics.push(
                    ProjectAnalyzeDiagnostic::new_deprecated_license(license.to_string())
                        .with_range(range),
                )
            }
        }

        PackageAnalyzeResult { diagnostics }
    }

    fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
