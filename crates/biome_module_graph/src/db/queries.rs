use crate::css_module_info::traverse::CssClassStep;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{CssModuleInfo, JsModuleInfo, ModuleDb};

/// Extracts the JS module info from a `ModuleInfo` input, if it is a JS module.
///
/// This is a tracked function: Salsa records that it reads `module.kind(db)`.
/// When the kind changes (file re-parsed), downstream consumers are invalidated.
#[salsa::tracked(no_eq)]
pub fn js_module_info(db: &dyn ModuleDb, module: ModuleInfo) -> Option<JsModuleInfo> {
    match module.kind(db) {
        ModuleInfoKind::Js(info) => Some(info),
        _ => None,
    }
}

/// Extracts the CSS module info from a `ModuleInfo` input, if it is a CSS module.
#[salsa::tracked(no_eq)]
pub fn css_module_info(db: &dyn ModuleDb, module: ModuleInfo) -> Option<CssModuleInfo> {
    match module.kind(db) {
        ModuleInfoKind::Css(info) => Some(info),
        _ => None,
    }
}

/// Returns CSS class steps for a JS module by traversing its direct CSS imports.
///
/// Tracked: depends on `js_module_info(db, module)` and on the CSS modules it
/// imports. If any of those change, this recomputes.
#[salsa::tracked(no_eq)]
pub fn css_classes_for_module(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<CssClassStep> {
    let Some(js_info) = js_module_info(db, module) else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for import_path in js_info.static_import_paths.values() {
        if let Some(path) = import_path.as_path()
            && let Some(target) = db.module_for_path(path)
            && let Some(css_info) = css_module_info(db, target)
        {
            results.push(CssClassStep {
                css_path: path.to_path_buf(),
                css_classes: css_info.classes.clone(),
            });
        }
    }

    results
}
