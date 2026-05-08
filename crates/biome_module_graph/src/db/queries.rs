use crate::css_module_info::traverse::CssClassStep;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{CssModuleInfo, JsExport, JsModuleInfo, JsOwnExport, ModuleDb};
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use std::collections::BTreeSet;

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
        if let Some(path) = import_path.as_path() {
            if let Some(target) = db.module_for_path(path) {
                if let Some(css_info) = css_module_info(db, target) {
                    results.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }
        }
    }

    results
}

/// Follows re-exports across modules to find the original definition of a symbol.
///
/// Not tracked — takes a `&str` parameter which isn't a Salsa type. This is
/// intentional: symbol-level lookups are cheap once the per-module data is
/// resolved. The expensive part (parsing, resolving imports) is tracked at the
/// module level via `js_module_info`.
pub(crate) fn find_exported_symbol_with_seen_paths(
    db: &dyn ModuleDb,
    module: JsModuleInfo,
    symbol_name: &str,
) -> Option<(JsModuleInfo, JsOwnExport)> {
    let mut seen_paths = BTreeSet::new();
    let mut stack = vec![(module, symbol_name.to_string())];

    while let Some((module, symbol_name)) = stack.pop() {
        match &module.exports.get(symbol_name.as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return Some((module.clone(), own_export.clone()));
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => {
                        break;
                    }
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.js_module_info_for_path(path) {
                                    stack.push((module, lookup));
                                    continue;
                                }
                            }
                            _ => break,
                        }
                    }
                    ImportSymbol::Default => {
                        if let Ok(path) = reexport.import.resolved_path.as_deref()
                            && let Some(module) = db.js_module_info_for_path(path)
                        {
                            stack.push((module, symbol_name));
                            continue;
                        }
                    }
                }
            }
            None => {
                for reexport in module.blanket_reexports.iter() {
                    if let Ok(path) = reexport.import.resolved_path.as_deref()
                        && seen_paths.insert(path.to_path_buf())
                        && let Some(module) = db.js_module_info_for_path(path)
                    {
                        stack.push((module, symbol_name.clone()));
                    }
                }
            }
        }
    }

    None
}

/// Finds an exported symbol by `symbol_name` as exported by `module`.
///
/// Follows re-exports if necessary.
pub(crate) fn find_exported_symbol(
    db: &dyn ModuleDb,
    module: &JsModuleInfo,
    symbol_name: &str,
) -> Option<JsOwnExport> {
    find_exported_symbol_with_seen_paths(db, module.clone(), symbol_name)
        .map(|(_, export)| export.clone())
}

pub fn find_jsdoc_for_exported_symbol(
    db: &dyn ModuleDb,
    module: &JsModuleInfo,
    symbol_name: &str,
) -> Option<JsdocComment> {
    find_exported_symbol_with_seen_paths(db, module.clone(), symbol_name).and_then(
        |(module, export)| match export {
            JsOwnExport::Binding(binding_range) => module
                .semantic_model
                .as_binding_by_range(binding_range)
                .and_then(|binding| binding.jsdoc().cloned()),
            JsOwnExport::Type(_) => None,
            JsOwnExport::Namespace(reexport) => reexport
                .export_range
                .and_then(|range| module.semantic_model.export_jsdoc(range).cloned()),
        },
    )
}
