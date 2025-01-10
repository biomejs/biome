use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use biome_fs::{BiomePath, FileSystem, PathKind};
use biome_js_syntax::AnyJsRoot;
use biome_project_layout::ProjectLayout;
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{CachedPath, ResolveOptions, ResolverGeneric};
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
    imports: HashMap<Utf8PathBuf, ModuleImports, FxBuildHasher>,
    path_info: HashMap<PathBuf, Option<PathKind>>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: Option<Utf8PathBuf>,
}

impl DependencyGraph {
    pub fn imports_for_path(&self, path: &Utf8Path) -> Option<ModuleImports> {
        self.imports.pin().get(path).cloned()
    }

    pub fn update_imports_for_js_paths(
        &self,
        fs: &dyn FileSystem,
        project_layout: &ProjectLayout,
        added_paths: &[BiomePath],
        removed_paths: &[BiomePath],
        get_js_syntax_for_path: impl Fn(&Utf8Path) -> Option<AnyJsRoot>,
    ) {
        let resolver_cache = Arc::new(ResolverCache::new(
            fs,
            project_layout,
            self,
            added_paths,
            removed_paths,
        ));
        let resolver = ResolverGeneric::new_with_cache(
            resolver_cache.clone(),
            ResolveOptions::default().with_symbolic_link(false),
        );

        let imports = self.imports.pin();
        for path in added_paths {
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
        let path_info = self.path_info.pin();
        for path in paths.pin().iter() {
            path_info.insert(path.to_path_buf(), path.kind());
        }

        for removed_path in removed_paths {
            path_info.remove(removed_path.as_std_path());
        }
    }

    pub(crate) fn path_kind(&self, path: &Path) -> Option<PathKind> {
        self.path_info.pin().get(path).copied().flatten()
    }
}

#[cfg(test)]
#[path = "dependency_graph.tests.rs"]
mod tests;
