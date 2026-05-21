use crate::{CssModuleInfo, HtmlModuleInfo, JsModuleInfo, ModuleInfo, ModuleInfoKind};
use biome_db::Db;
use camino::{Utf8Path, Utf8PathBuf};

pub mod project_database;
pub mod queries;

/// Extends `Db` with module-graph-specific lookups.
#[salsa::db]
pub trait ModuleDb: Db {
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
