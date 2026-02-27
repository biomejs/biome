//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! The module graph is instantiated and updated inside the Workspace Server.

mod fs_proxy;

use std::{collections::BTreeSet, ops::Deref};

use crate::css_module_info::{CssModuleInfo, CssModuleVisitor, SerializedCssModuleInfo};
use crate::{
    JsExport, JsModuleInfo, JsOwnExport, ModuleDiagnostic, SerializedJsModuleInfo,
    js_module_info::JsModuleVisitor,
};
use biome_css_syntax::AnyCssRoot;
use biome_fs::BiomePath;
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use biome_project_layout::ProjectLayout;
use biome_resolver::{FsWithResolverProxy, PathInfo};
use camino::{Utf8Path, Utf8PathBuf};
pub(crate) use fs_proxy::ModuleGraphFsProxy;
use papaya::{HashMap, HashMapRef, LocalGuard};
use rustc_hash::{FxBuildHasher, FxHashSet};

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node",
];

/// Data structure for tracking imports and exports across files.
///
/// The module graph is also augmented with type information, allowing types
/// to be looked up from imports as well.
///
/// The module graph is simply a flat mapping from paths to module info
/// structures. This approach makes both lookups easy and makes it very easy for
/// us to invalidate part of the graph when there are file system changes.
#[derive(Debug, Default)]
pub struct ModuleGraph {
    /// Cached module info per file.
    data: HashMap<Utf8PathBuf, ModuleInfo, FxBuildHasher>,

    /// Cache that tracks the presence of files, directories, and symlinks
    /// across the project.
    path_info: HashMap<Utf8PathBuf, Option<PathInfo>>,
}

impl ModuleGraph {
    /// Returns whether the given `path` is indexed in the module graph.
    pub fn contains(&self, path: &Utf8Path) -> bool {
        self.data.pin().contains_key(path)
    }

    /// Returns the module info, such as imports and exports and their types,
    /// for the given `path`.
    pub fn js_module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.data.pin().get(path).and_then(|info| match info {
            ModuleInfo::Js(module_info) => Some(module_info.clone()),
            _ => None,
        })
    }

    /// Returns the data of the module graph in test
    pub fn data(&self) -> HashMapRef<'_, Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard<'_>> {
        self.data.pin()
    }

    /// Updates the module graph to add, update, or remove files.
    ///
    /// Only JavaScript/TypeScript files need to be provided as part of
    /// `added_or_updated_paths` and `removed_paths`. Manifests are expected to
    /// be resolved through the `project_layout`. As such, the `project_layout`
    /// must have been updated before calling this method.
    ///
    /// Returns the dependencies of all the paths that were added or updated.
    pub fn update_graph_for_js_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(
            &BiomePath,
            AnyJsRoot,
            std::sync::Arc<biome_js_semantic::SemanticModel>,
        )],
        enable_type_inference: bool,
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for (path, _, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                let mut inserted = false;
                path_info.get_or_insert_with(path.to_path_buf(), || {
                    inserted = true;
                    fs.path_info(path).ok()
                });
                if !inserted {
                    break;
                }
                parent = path.parent();
            }
        }

        let fs_proxy = ModuleGraphFsProxy::new(fs, self, project_layout);
        let mut dependencies = ModuleDependencies::default();
        let mut diagnostics = Vec::new();

        // Traverse all the added and updated paths and insert their module
        // info.
        let modules = self.data.pin();
        for (path, root, semantic_model) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = JsModuleVisitor::new(
                root.clone(),
                directory,
                &fs_proxy,
                semantic_model.clone(),
                enable_type_inference,
            );

            let module_info = visitor.collect_info();
            for import_path in module_info.all_import_paths() {
                if let Some(path) = import_path.as_path() {
                    dependencies.insert(path.to_path_buf());
                }
            }

            for diagnostic in module_info.diagnostics() {
                diagnostics.push(diagnostic.clone());
            }

            modules.insert(path.to_path_buf(), module_info.into());
        }

        (dependencies, diagnostics)
    }

    pub fn update_graph_for_css_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(&BiomePath, AnyCssRoot)],
        _semantic_model: Option<&biome_css_semantic::model::SemanticModel>,
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for (path, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                let mut inserted = false;
                path_info.get_or_insert_with(path.to_path_buf(), || {
                    inserted = true;
                    fs.path_info(path).ok()
                });
                if !inserted {
                    break;
                }
                parent = path.parent();
            }
        }

        let fs_proxy = ModuleGraphFsProxy::new(fs, self, project_layout);
        let mut dependencies = ModuleDependencies::default();
        let diagnostics = Vec::new();

        // Traverse all the added and updated paths and insert their module
        // info.
        let modules = self.data.pin();

        for (path, root) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = CssModuleVisitor::new(root.clone(), directory, &fs_proxy);

            let module = visitor.visit();

            for (_, path) in module.0.imports.deref() {
                if let Some(path) = path.resolved_path.as_path() {
                    dependencies.insert(path.to_path_buf());
                }
            }

            modules.insert(path.to_path_buf(), module.into());
        }

        (dependencies, diagnostics)
    }

    pub fn update_graph_for_removed_paths(&self, removed_paths: &[&BiomePath]) {
        let modules = self.data.pin();
        // Clean up removed paths from the module graph and path info cache.
        let path_info = self.path_info.pin();
        // Clean up removed paths.
        for removed_path in removed_paths {
            modules.remove(removed_path.as_path());
            path_info.remove(removed_path.as_path());
        }
    }

    pub fn get_or_insert_path_info(
        &self,
        path: &Utf8Path,
        fs: &dyn FsWithResolverProxy,
    ) -> Option<PathInfo> {
        self.path_info
            .pin()
            .get_or_insert_with(path.to_path_buf(), || fs.path_info(path).ok())
            .clone()
    }

    /// Unloads all paths from the graph within the given `path`.
    ///
    /// This method works both for unloading folders as well as individual
    /// files.
    pub fn unload_path(&self, path: &Utf8Path) {
        let data = self.data.pin();
        for indexed_path in data.keys() {
            if indexed_path.starts_with(path) {
                data.remove(indexed_path);
            }
        }
    }

    /// Finds an exported symbol by `symbol_name` as exported by `module`.
    ///
    /// Follows re-exports if necessary.
    pub(crate) fn find_exported_symbol(
        &self,
        module: &JsModuleInfo,
        symbol_name: &str,
    ) -> Option<JsOwnExport> {
        let data = self.data.pin();
        let mut seen_paths = BTreeSet::new();

        find_exported_symbol_with_seen_paths(&data, module, symbol_name, &mut seen_paths)
            .map(|(_, export)| export.clone())
    }

    /// Finds an exported symbol by `symbol_name` as exported by `module`.
    ///
    /// Follows re-exports if necessary.
    pub(crate) fn find_jsdoc_for_exported_symbol(
        &self,
        module: &JsModuleInfo,
        symbol_name: &str,
    ) -> Option<JsdocComment> {
        let data = self.data.pin();
        let mut seen_paths = BTreeSet::new();

        find_exported_symbol_with_seen_paths(&data, module, symbol_name, &mut seen_paths).and_then(
            |(module, export)| match export {
                JsOwnExport::Binding(binding_range) => module
                    .binding_type_data(*binding_range)
                    .and_then(|data| data.jsdoc.clone()),
                JsOwnExport::Type(_) => None,
            },
        )
    }
}

#[derive(Debug)]
pub enum ModuleInfo {
    Js(JsModuleInfo),
    Css(CssModuleInfo),
}

impl From<JsModuleInfo> for ModuleInfo {
    fn from(value: JsModuleInfo) -> Self {
        Self::Js(value)
    }
}

impl From<CssModuleInfo> for ModuleInfo {
    fn from(value: CssModuleInfo) -> Self {
        Self::Css(value)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SerializedModuleInfo {
    Js(SerializedJsModuleInfo),
    Css(SerializedCssModuleInfo),
}

impl SerializedModuleInfo {
    pub fn as_js_module_info(&self) -> Option<&SerializedJsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&SerializedCssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }
}

impl ModuleInfo {
    pub fn dump(&self) -> SerializedModuleInfo {
        match self {
            Self::Js(module) => SerializedModuleInfo::Js(module.dump()),
            Self::Css(module) => SerializedModuleInfo::Css(module.dump()),
        }
    }

    pub fn as_js_module_info(&self) -> Option<&JsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&CssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }
}

fn find_exported_symbol_with_seen_paths<'a>(
    data: &'a HashMapRef<Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard>,
    module: &'a JsModuleInfo,
    symbol_name: &str,
    seen_paths: &mut BTreeSet<&'a Utf8Path>,
) -> Option<(&'a JsModuleInfo, &'a JsOwnExport)> {
    match module.exports.get(symbol_name) {
        Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
            Some((module, own_export))
        }
        Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
            if reexport.import.symbol == ImportSymbol::All {
                // TODO: Follow namespace exports.
                None
            } else {
                match reexport.import.resolved_path.as_deref() {
                    Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                        if let ModuleInfo::Js(module) = module {
                            find_exported_symbol_with_seen_paths(
                                data,
                                module,
                                symbol_name,
                                seen_paths,
                            )
                        } else {
                            None
                        }
                    }),
                    _ => None,
                }
            }
        }
        None => module.blanket_reexports.iter().find_map(|reexport| {
            match reexport.import.resolved_path.as_deref() {
                Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                    if let ModuleInfo::Js(module) = module {
                        find_exported_symbol_with_seen_paths(data, module, symbol_name, seen_paths)
                    } else {
                        None
                    }
                }),
                _ => None,
            }
        }),
    }
}

/// Represents all the files that are imported/depended on by a module.
#[derive(Debug, Default)]
pub struct ModuleDependencies(FxHashSet<Utf8PathBuf>);

impl ModuleDependencies {
    /// Adds a dependency to the module dependencies, if it wasn't added yet.
    pub fn insert(&mut self, dependency_path: Utf8PathBuf) {
        self.0.insert(dependency_path);
    }
}

impl AsRef<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn as_ref(&self) -> &FxHashSet<Utf8PathBuf> {
        &self.0
    }
}

impl Deref for ModuleDependencies {
    type Target = FxHashSet<Utf8PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn from(dependencies: FxHashSet<Utf8PathBuf>) -> Self {
        Self(dependencies)
    }
}

impl FromIterator<Utf8PathBuf> for ModuleDependencies {
    fn from_iter<T: IntoIterator<Item = Utf8PathBuf>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for ModuleDependencies {
    type Item = Utf8PathBuf;

    type IntoIter = <FxHashSet<Utf8PathBuf> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
