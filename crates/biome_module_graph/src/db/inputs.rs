use crate::module_graph::ModuleInfoKind;
use crate::{CssModuleInfo, HtmlModuleInfo, JsModuleInfo, ModuleInfo};
use biome_db::Db;
use camino::Utf8Path;

/// Extends `Db` with module-graph-specific lookups.
#[salsa::db]
pub trait ModuleDb: Db {
    /// Given a path, it retrieves its corresponding module info.
    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo>;

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
}
