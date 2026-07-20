//! Salsa database traits and tracked module-graph queries.

use crate::{CssModuleInfo, HtmlModuleInfo, JsModuleInfo, ModuleInfo, ModuleInfoKind};
pub use biome_js_type_info::TypeDb;
use biome_js_type_info::resolved::InferredModuleKey;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::plumbing::{AsId, FromId};

pub mod queries;
mod type_inference;

/// Tracks changes with which module is associated with each file path in a
/// [`ModuleDb`].
///
/// Each [`ModuleInfo`] is a Salsa input, but the database stores the association
/// between file paths and `ModuleInfo` values outside Salsa. Salsa cannot detect
/// when a path is added, removed, or associated with a different `ModuleInfo`.
/// Read operations therefore read this generation, and write operations
/// increment it. This allows Salsa to rerun queries after those associations
/// change.
///
/// The generation is a singleton because it represents all modules in one
/// database, rather than an individual module. Each Salsa storage has one
/// generation, so all read and write operations use the same value. The value
/// can be retrieved with `ModuleGraphGeneration::get(db)` without passing a
/// separate input through every API.
///
/// The generation is initialized once for a Salsa storage and is not purged or
/// reset while that storage remains alive. Updating it changes the `u64` on the
/// existing input; it does not create another `ModuleGraphGeneration` input.
/// Dropping the Salsa storage drops the singleton, and a new storage starts
/// with a new value.
#[salsa::input(singleton)]
pub struct ModuleGraphGeneration {
    pub value: u64,
}

/// Extends `TypeDb` with module-graph-specific lookups.
#[salsa::db]
pub trait ModuleDb: TypeDb {
    /// Returns the generation used when reading modules by file path.
    ///
    /// A database that can add, remove, or change the module associated with a
    /// path after read operations begin must override this method unless Salsa
    /// already tracks those changes. The default is only suitable when
    /// those associations do not change while they are being read.
    ///
    /// # Read operations
    ///
    /// A read operation must read this generation before directly reading the
    /// module paths or the [`ModuleInfo`] associated with a path. An operation
    /// that calls [`Self::module_for_path`] or [`Self::for_each_module`] does not
    /// need to read it again because those methods already do so.
    ///
    /// Reading tracked fields from an already known [`ModuleInfo`] does not
    /// require the generation. Read operations that intentionally bypass Salsa
    /// tracking must not be used from Salsa queries or for module analysis.
    ///
    /// # Write operations
    ///
    /// A write operation must update the generation when it adds or removes a
    /// module or changes the [`ModuleInfo`] associated with a path. When the
    /// module data is stored outside Salsa, start the Salsa write before
    /// changing the data and update the generation after the change.
    fn module_graph_generation(&self) -> u64 {
        0
    }

    /// Given a path, it retrieves its corresponding module info.
    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo>;

    /// Iterates over all indexed modules.
    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind));

    /// Returns whether the given `path` is indexed.
    fn contains(&self, path: &Utf8Path) -> bool {
        self.module_for_path(path).is_some()
    }

    /// Returns the JS module info for the given `path`.
    fn js_module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.module_for_path(path)
            .and_then(|info| match info.kind(self) {
                ModuleInfoKind::Js(module_info) => Some(module_info.clone()),
                _ => None,
            })
    }

    /// Returns the CSS module info for the given `path`.
    fn css_module_info_for_path(&self, path: &Utf8Path) -> Option<CssModuleInfo> {
        self.module_for_path(path)
            .and_then(|info| match info.kind(self) {
                ModuleInfoKind::Css(module_info) => Some(module_info.clone()),
                _ => None,
            })
    }

    /// Returns the HTML module info for the given `path`.
    fn html_module_info_for_path(&self, path: &Utf8Path) -> Option<HtmlModuleInfo> {
        self.module_for_path(path)
            .and_then(|info| match info.kind(self) {
                ModuleInfoKind::Html(module_info) => Some(module_info.clone()),
                _ => None,
            })
    }

    /// Returns the module info kind for the given `path`.
    fn module_info_for_path(&self, path: &Utf8Path) -> Option<ModuleInfoKind> {
        self.module_for_path(path)
            .map(|info| info.kind(self).clone())
    }

    /// Collects all module paths and their kinds.
    fn all_modules(&self) -> Vec<(Utf8PathBuf, ModuleInfoKind)> {
        let mut result = Vec::new();
        self.for_each_module(&mut |path, kind| {
            result.push((path.to_path_buf(), kind.clone()));
        });
        result
    }
}

/// Resolves a module key while rejecting stale module handles.
pub fn module_for_key(db: &dyn ModuleDb, module_key: InferredModuleKey) -> Option<ModuleInfo> {
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    (InferredModuleKey::new(current.as_id()) == module_key).then_some(current)
}
