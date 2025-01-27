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
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{ResolveError, ResolveOptions, ResolverGeneric};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

use crate::{import_visitor::ImportVisitor, resolver_cache::ResolverCache};

/// Data structure for tracking imports across files.
///
/// The dependency graph is simply a flat mapping from paths to module imports.
/// This approach makes both lookups easy and makes it very easy for us to
/// invalidate part of the graph when there are file system changes.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// Cached imports per file.
    imports: HashMap<Utf8PathBuf, ModuleImports, FxBuildHasher>,

    /// Cache that tracks the presence of files, directories, and symlinks
    /// across the project.
    path_info: HashMap<Utf8PathBuf, Option<PathKind>>,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleImports {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the identifier found in the import statement to the absolute
    /// path it resolves to. The resolved path may be looked up as key in the
    /// [DependencyGraphModel::modules] map, although it is not required to
    /// exist (for instance, if the path is outside the project's scope).
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
}

impl ModuleImports {
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
    pub fn imports_for_path(&self, path: &Utf8Path) -> Option<ModuleImports> {
        self.imports.pin().get(path).cloned()
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
    pub fn update_imports_for_js_paths(
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
        let resolver =
            ResolverGeneric::new_with_cache(resolver_cache.clone(), ResolveOptions::default());

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
                };
                parent = path.parent();
            }
        }

        // Traverse all the added and updated paths and insert their resolved
        // imports.
        let imports = self.imports.pin();
        for path in added_or_updated_paths {
            let Some(root) = get_js_syntax_for_path(path) else {
                continue;
            };

            let directory = path.parent().unwrap_or(path);
            let visitor = ImportVisitor::new(root, directory, &resolver);
            let module_imports = visitor.find_module_imports();
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

#[cfg(test)]
#[path = "dependency_graph.tests.rs"]
mod tests;
