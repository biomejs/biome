mod package_json;
mod tsconfig_json;
use crate::{LICENSE_LIST, Manifest, Package, PackageAnalyzeResult, ProjectAnalyzeDiagnostic};
use biome_parser::AnyParse;
use biome_rowan::Language;
pub use package_json::{Dependencies, PackageJson, PackageType, Version};
pub use tsconfig_json::TsConfigJson;

#[derive(Default, Debug, Clone)]
/// A Node.js project.
pub struct NodeJsPackage {
    /// The `package.json` manifest
    pub manifest: Option<PackageJson>,
    /// The raw manifest
    pub raw_manifest: Option<AnyParse>,
    /// Diagnostics emitted during the operations
    pub diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    /// The `tsconfig.json` manifest
    pub tsconfig: Option<TsConfigJson>,
}

impl NodeJsPackage {
    pub fn deserialize_tsconfig(&mut self, content: &ProjectLanguageRoot<TsConfigJson>) {
        let tsconfig = TsConfigJson::deserialize_manifest(content);
        let (tsconfig, deserialize_diagnostics) = tsconfig.consume();
        self.tsconfig = tsconfig;
        self.diagnostics = deserialize_diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect();
    }

    pub fn to_deserialized_manifest(&self) -> Option<PackageJson> {
        self.raw_manifest
            .as_ref()
            .and_then(|root| {
                let node = root.tree();
                let deserialized = <Self as Package>::Manifest::deserialize_manifest(&node);
                let (manifest, _) = deserialized.consume();
                manifest
            })
            .or_else(|| self.manifest.clone())
    }
}

pub(crate) type ProjectLanguageRoot<M> = <<M as Manifest>::Language as Language>::Root;

impl Package for NodeJsPackage {
    type Manifest = PackageJson;

    fn insert_serialized_manifest(&mut self, content: &ProjectLanguageRoot<Self::Manifest>) {
        let deserialized = Self::Manifest::deserialize_manifest(content);
        let (manifest, diagnostics) = deserialized.consume();
        self.manifest = manifest;
        self.diagnostics = diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect();
    }

    fn insert_raw_manifest(&mut self, json_root: &AnyParse) {
        self.raw_manifest = Some(json_root.clone());
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
                diagnostics
                    .push(ProjectAnalyzeDiagnostic::new_invalid_license(license).with_range(range))
            } else if !LICENSE_LIST.is_deprecated(license) {
                diagnostics.push(
                    ProjectAnalyzeDiagnostic::new_deprecated_license(license).with_range(range),
                )
            }
        }

        PackageAnalyzeResult { diagnostics }
    }

    fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
