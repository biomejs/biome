mod visitor;

use biome_resolver::ResolvedPath;
use biome_rowan::Text;
use indexmap::IndexMap;
use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub(crate) use visitor::CssModuleVisitor;

/// Information restricted to a single module in the [ModuleGraph].
#[derive(Clone, Debug)]
pub struct CssModuleInfo(pub(super) Arc<CssModuleInfoInner>);

impl Deref for CssModuleInfo {
    type Target = CssModuleInfoInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl CssModuleInfo {
    pub(crate) fn new(imports: CssImports) -> Self {
        let info = CssModuleInfoInner { imports };
        Self(Arc::new(info))
    }

    pub(crate) fn dump(&self) -> SerializedCssModuleInfo {
        SerializedCssModuleInfo {
            imports: self
                .0
                .imports
                .iter()
                .map(|(_, static_import)| static_import.specifier.to_string())
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CssModuleInfoInner {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the import specifier to a [CssImport] with the absolute path
    /// it resolves to. The resolved path may be looked up as key in the
    /// [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    pub imports: CssImports,
}

#[derive(Debug, Default, Clone)]
pub struct CssImports(pub(crate) IndexMap<Text, CssImport>);

impl Deref for CssImports {
    type Target = IndexMap<Text, CssImport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CssImports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents an import to one or more symbols from an external path.
///
/// It could point to any kind of resource, such as JavaScript files, CSS files,
/// images, and so on.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CssImport {
    /// The specifier for the imported as it appeared in the source text.
    pub specifier: Text,

    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: ResolvedPath,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedCssModuleInfo {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the local imported name to the absolute path it resolves to.
    pub imports: BTreeSet<String>,
}
