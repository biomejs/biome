//! Dependency graph tracking imports across files.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! The dependency graph is instantiated and updated inside the Workspace
//! Server.
use std::{collections::BTreeMap, sync::Arc};

use biome_fs::{BiomePath, FileSystem, PathKind};
use biome_js_syntax::AnyJsRoot;
use biome_project_layout::ProjectLayout;
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{EnforceExtension, ResolveError, ResolveOptions, ResolverGeneric};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

use crate::{module_visitor::ModuleVisitor, resolver_cache::ResolverCache};

/// Data structure for tracking imports and exports across files.
///
/// The dependency graph is simply a flat mapping from paths to module imports.
/// This approach makes both lookups easy and makes it very easy for us to
/// invalidate part of the graph when there are file system changes.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// Cached dependency data per file.
    data: HashMap<Utf8PathBuf, ModuleDependencyData, FxBuildHasher>,

    /// Cache that tracks the presence of files, directories, and symlinks
    /// across the project.
    path_info: HashMap<Utf8PathBuf, Option<PathKind>>,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleDependencyData {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the identifier found in the import statement to the absolute
    /// path it resolves to. The resolved path may be looked up as key in the
    /// [DependencyGraphModel::modules] map, although it is not required to
    /// exist (for instance, if the path is outside the project's scope).
    ///
    /// Note that re-exports may introduce additional dependencies, because they
    /// import another module and immediately re-export from that module.
    /// Re-exports are tracked as part of [Self::exports] and
    /// [Self::blanket_reexports].
    pub static_imports: BTreeMap<String, Import>,

    /// Map of all dynamic imports found in the module for which the import
    /// identifier could be statically determined.
    ///
    /// Dynamic imports for which the identifier cannot be statically determined
    /// (for instance, because a template string with variables is used) will be
    /// omitted from this map.
    ///
    /// Maps from the identifier found in the import expression to the absolute
    /// path it resolves to. The resolved path may be looked up as key in the
    /// [DependencyGraphModel::modules] map, although it is not required to
    /// exist (for instance, if the path is outside the project's scope).
    ///
    /// `require()` expressions in CommonJS sources are also included with the
    /// dynamic imports.
    pub dynamic_imports: BTreeMap<String, Import>,

    /// Map of exports from the module.
    ///
    /// The keys are the names of the exports, where "default" is used for the
    /// default export. See [Export] for information tracked per export.
    ///
    /// Re-exports are tracked in this map as well. They exception are "blanket"
    /// re-exports, such as `export * from "other-module"`. Those are tracked in
    /// [Self::forwarding_exports] instead.
    pub exports: BTreeMap<Text, Export>,

    /// Re-exports that apply to all symbols from another module, without
    /// assigning a name to them.
    pub blanket_reexports: Vec<ReexportAll>,
}

impl ModuleDependencyData {
    /// Allows draining a single entry from the imports.
    ///
    /// Returns a `(specifier, import)` pair from either the static or dynamic
    /// imports, whichever is non-empty. Returns `None` if both are empty.
    ///
    /// Using this method allows for consuming the struct while iterating over
    /// it, without necessarily turning the entire struct into an iterator at
    /// once.
    pub fn drain_one(&mut self) -> Option<(String, Import)> {
        if self.static_imports.is_empty() {
            self.dynamic_imports.pop_first()
        } else {
            self.static_imports.pop_first()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: Result<Utf8PathBuf, ResolveError>,
}

impl DependencyGraph {
    /// Returns the dependency data, such as imports and exports, for the
    /// given `path`.
    pub fn dependency_data_for_path(&self, path: &Utf8Path) -> Option<ModuleDependencyData> {
        self.data.pin().get(path).cloned()
    }

    /// Updates the dependency graph to add, update, or remove files.
    ///
    /// Only JavaScript/TypeScript files need to be provided as part of
    /// `added_or_updated_paths` and `removed_paths`. Manifests are expected to
    /// be resolved through the `project_layout`. As such, the `project_layout`
    /// must have been updated before calling this method.
    ///
    /// `get_js_syntax_for_path` is a callback that may be called for any of the
    /// files given in `added_or_updated_paths`, and it should return the syntax
    /// root for each of them. If a file is already removed, or is inaccessible,
    /// by the time `get_js_syntax_for_path` is called for it, `None` must be
    /// returned. Error reporting, if necessary, should be handled by the
    /// workspace server instead.
    pub fn update_graph_for_js_paths(
        &self,
        fs: &dyn FileSystem,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[BiomePath],
        removed_paths: &[BiomePath],
        get_js_syntax_for_path: impl Fn(&Utf8Path) -> Option<AnyJsRoot>,
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
            extensions: vec![
                ".js".into(),
                ".jsx".into(),
                ".mjs".into(),
                ".cjs".into(),
                ".ts".into(),
                ".tsx".into(),
                ".mts".into(),
                ".cts".into(),
                ".json".into(),
                ".node".into(),
            ],
            ..Default::default()
        };
        let resolver = ResolverGeneric::new_with_cache(resolver_cache.clone(), resolve_options);

        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for path in added_or_updated_paths {
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
        for path in added_or_updated_paths {
            let Some(root) = get_js_syntax_for_path(path) else {
                continue;
            };

            let directory = path.parent().unwrap_or(path);
            let visitor = ModuleVisitor::new(root, directory, &resolver);
            let module_imports = visitor.collect_data();
            imports.insert(path.to_path_buf(), module_imports);
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
}

/// Information tracked for every export.
///
/// Exports come in three varieties: "own" exports that are defined in the
/// module itself, re-exports for named exports, and re-exports that apply to
/// all symbols from another module.
#[derive(Clone, Debug, PartialEq)]
pub enum Export {
    /// An export that is defined in this module.
    Own(OwnExport),
    /// An export that is re-exported by this module, but which is defined
    /// within another module.
    ///
    /// E.g. `export { someSymbol } from "other-module"`.
    Reexport(Import),
    /// An export that creates an alias for all symbols from another module.
    ///
    /// E.g. `export { * as alias } from "other-module"`.
    ReexportAll(ReexportAll),
}

/// Information tracked for every "own" export.
#[derive(Clone, Debug, PartialEq)]
pub struct OwnExport {
    /// Optional JSDoc comment associated with the export.
    ///
    /// The comment is trimmed and normalized to remove the trivia of the
    /// comment.
    ///
    /// ## Example
    ///
    /// Assuming the following export:
    ///
    /// ```ts
    /// /**
    ///  * Magic constant of fooness.
    ///  *
    ///  * For if you want more ways to write 1.
    ///  */
    /// export const FOO = 1;
    /// ```
    ///
    /// The comment would be:
    /// `"Magic constant of fooness.\n\nFor if you want more ways to write 1."`.
    pub jsdoc_comment: Option<String>,
}

/// Information about an export statement that re-exports all symbols from
/// another module.
#[derive(Clone, Debug, PartialEq)]
pub struct ReexportAll {
    /// Import from which the symbols are being re-exported.
    pub import: Import,
}

#[cfg(test)]
#[path = "dependency_graph.tests.rs"]
mod tests;
