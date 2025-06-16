//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! The module graph is instantiated and updated inside the Workspace Server.

mod fs_proxy;

use std::collections::BTreeSet;

use biome_fs::BiomePath;
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use biome_project_layout::ProjectLayout;
use biome_resolver::{FsWithResolverProxy, PathInfo};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{HashMap, HashMapRef, LocalGuard};
use rustc_hash::FxBuildHasher;

use crate::{JsExport, JsModuleInfo, JsOwnExport, js_module_info::JsModuleVisitor};

pub(crate) use fs_proxy::ModuleGraphFsProxy;

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
    // TODO: When we want to generalise the module graph across languages,
    //       we should insert a `ModuleInfo` enum with variants such as
    //       `Js(JsModuleInfo)` and those for other languages.
    data: HashMap<Utf8PathBuf, JsModuleInfo, FxBuildHasher>,

    /// Cache that tracks the presence of files, directories, and symlinks
    /// across the project.
    path_info: HashMap<Utf8PathBuf, Option<PathInfo>>,
}

impl ModuleGraph {
    /// Returns the module info, such as imports and exports and their types,
    /// for the given `path`.
    pub fn module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.data.pin().get(path).cloned()
    }

    /// Returns the data of the module graph in test
    pub fn data(&self) -> HashMapRef<Utf8PathBuf, JsModuleInfo, FxBuildHasher, LocalGuard> {
        self.data.pin()
    }

    /// Updates the module graph to add, update, or remove files.
    ///
    /// Only JavaScript/TypeScript files need to be provided as part of
    /// `added_or_updated_paths` and `removed_paths`. Manifests are expected to
    /// be resolved through the `project_layout`. As such, the `project_layout`
    /// must have been updated before calling this method.
    pub fn update_graph_for_js_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(&BiomePath, AnyJsRoot)],
        removed_paths: &[&BiomePath],
    ) {
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

        // Traverse all the added and updated paths and insert their module
        // info.
        let imports = self.data.pin();
        for (path, root) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = JsModuleVisitor::new(root.clone(), directory, &fs_proxy);
            imports.insert(path.to_path_buf(), visitor.collect_info());
        }

        // Clean up removed paths.
        for removed_path in removed_paths {
            imports.remove(removed_path.as_path());
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
                JsOwnExport::Binding(binding_id) => {
                    module.bindings[binding_id.index()].jsdoc.clone()
                }
                JsOwnExport::Type(_) => None,
            },
        )
    }
}

fn find_exported_symbol_with_seen_paths<'a>(
    data: &'a HashMapRef<Utf8PathBuf, JsModuleInfo, FxBuildHasher, LocalGuard>,
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
                        find_exported_symbol_with_seen_paths(data, module, symbol_name, seen_paths)
                    }),
                    _ => None,
                }
            }
        }
        None => module.blanket_reexports.iter().find_map(|reexport| {
            match reexport.import.resolved_path.as_deref() {
                Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                    find_exported_symbol_with_seen_paths(data, module, symbol_name, seen_paths)
                }),
                _ => None,
            }
        }),
    }
}
