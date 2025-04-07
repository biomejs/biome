//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! The module graph is instantiated and updated inside the Workspace Server.
use std::{collections::BTreeSet, sync::Arc};

use biome_fs::{BiomePath, FileSystem, PathKind};
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::{Namespace, Type};
use biome_project_layout::ProjectLayout;
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{EnforceExtension, ResolveOptions, ResolverGeneric};
use papaya::{HashMap, HashMapRef, LocalGuard};
use rustc_hash::FxBuildHasher;

use crate::{
    Export,
    js_module_visitor::JsModuleVisitor,
    module_info::{ModuleInfo, OwnExport},
    resolver_cache::ResolverCache,
};

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts", ".json", ".node",
];

fn supported_extensions_owned() -> Vec<String> {
    let mut extensions = Vec::with_capacity(SUPPORTED_EXTENSIONS.len());
    for extension in SUPPORTED_EXTENSIONS {
        extensions.push((*extension).to_string());
    }
    extensions
}

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
    path_info: HashMap<Utf8PathBuf, Option<PathKind>>,
}

impl ModuleGraph {
    /// Returns the module info, such as imports and exports and their types,
    /// for the given `path`.
    pub fn module_info_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.data.pin().get(path).cloned()
    }

    /// Updates the module graph to add, update, or remove files.
    ///
    /// Only JavaScript/TypeScript files need to be provided as part of
    /// `added_or_updated_paths` and `removed_paths`. Manifests are expected to
    /// be resolved through the `project_layout`. As such, the `project_layout`
    /// must have been updated before calling this method.
    pub fn update_graph_for_js_paths(
        &self,
        fs: &dyn FileSystem,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(&BiomePath, Option<AnyJsRoot>)],
        removed_paths: &[BiomePath],
    ) {
        let resolver_cache = Arc::new(ResolverCache::new(
            fs,
            project_layout,
            self,
            added_or_updated_paths,
            removed_paths,
        ));
        let resolve_options = ResolveOptions {
            enforce_extension: EnforceExtension::Disabled,
            extensions: supported_extensions_owned(),
            ..Default::default()
        };
        let resolver = ResolverGeneric::new_with_cache(resolver_cache.clone(), resolve_options);

        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for (path, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                if path_info
                    .try_insert_with(path.to_path_buf(), || fs.path_kind(path).ok())
                    .is_err()
                {
                    break;
                }
                parent = path.parent();
            }
        }

        // Traverse all the added and updated paths and insert their resolved
        // imports.
        let imports = self.data.pin();
        for (path, root) in added_or_updated_paths
            .iter()
            .filter_map(|(path, root)| root.clone().map(|root| (path, root)))
        {
            let directory = path.parent().unwrap_or(path);
            let visitor = JsModuleVisitor::new(root, directory, &resolver);
            imports.insert(path.to_path_buf(), visitor.collect_info());
        }

        // Update our `path_info` cache so that future usages of the
        // `ResolverCache` get cache hits through [Self::path_kind()].
        let paths = resolver_cache.paths();
        for path in paths.pin().iter() {
            path_info.insert(path.path.to_path_buf(), path.kind());
        }

        // Clean up removed paths.
        for removed_path in removed_paths {
            imports.remove(removed_path.as_path());
            path_info.remove(removed_path.as_path());
        }
    }

    pub(crate) fn path_kind(&self, path: &Utf8Path) -> Option<PathKind> {
        self.path_info.pin().get(path).copied().flatten()
    }

    /// Finds an exported symbol by `symbol_name` as exported by `module`.
    ///
    /// Follows re-exports if necessary.
    pub(crate) fn find_exported_symbol(
        &self,
        module: &ModuleInfo,
        symbol_name: &str,
    ) -> Option<OwnExport> {
        let data = self.data.pin();
        let mut seen_paths = BTreeSet::new();

        fn find_exported_symbol_with_seen_paths<'a>(
            data: &'a HashMapRef<Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard>,
            module: &'a ModuleInfo,
            symbol_name: &str,
            seen_paths: &mut BTreeSet<&'a Utf8Path>,
        ) -> Option<OwnExport> {
            match module.exports.get(symbol_name) {
                Some(Export::Own(own_export) | Export::OwnType(own_export)) => {
                    Some(own_export.clone())
                }
                Some(Export::Reexport(import) | Export::ReexportType(import)) => {
                    match &import.resolved_path {
                        Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                            find_exported_symbol_with_seen_paths(
                                data,
                                module,
                                symbol_name,
                                seen_paths,
                            )
                        }),
                        _ => None,
                    }
                }
                Some(Export::ReexportAll(reexport_all)) => Some(OwnExport {
                    jsdoc_comment: reexport_all.jsdoc_comment.clone(),
                    ty: Type::Namespace(Box::new(Namespace(Box::new([])))),
                }),
                None => module.blanket_reexports.iter().find_map(|reexport| {
                    match &reexport.import.resolved_path {
                        Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                            find_exported_symbol_with_seen_paths(
                                data,
                                module,
                                symbol_name,
                                seen_paths,
                            )
                        }),
                        _ => None,
                    }
                }),
            }
        }

        find_exported_symbol_with_seen_paths(&data, module, symbol_name, &mut seen_paths)
    }
}

#[cfg(test)]
#[path = "module_graph.tests.rs"]
mod tests;
