//! This module represents the database queries used by the module graph.
//!
//! The queries are defined in terms of `ModuleInfo` inputs.
//!
//! The queries are tracked so that Salsa can invalidate them when the inputs
//! change.
//!
//! The queries are also interned, so that Salsa can reuse the same computation
//! when the inputs are the same.
//!
//! This module should contain only tracked functions, exposed to the consumers. Middle
//! functions that aren't queries should be moved somewhere else, unless they are used
//! directly by the tracked functions e.g. cycle detection

mod css;
mod type_inference;

use crate::{JsExport, JsExportedSymbolLookup, JsOwnExport, ModuleDb, ModuleInfo, ModuleInfoKind};
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;

pub use crate::db::type_inference::InferredModuleTypes;
pub use css::*;
pub use type_inference::*;

// #region EXPORTED TRACKED QUERIES

/// Finds the exported symbol with the given name, following re-exports.
#[salsa::tracked]
pub fn find_js_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> JsExportedSymbolLookup {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![symbol];
    let mut saw_unresolved_target = false;

    while let Some(symbol) = stack.pop() {
        let ModuleInfoKind::Js(module) = symbol.module(db).kind(db) else {
            continue;
        };
        match &module.exports.get(symbol.name(db).as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return JsExportedSymbolLookup::Found(own_export.clone());
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => break,
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.module_for_path(path) {
                                    stack.push(SymbolFromModuleInfo::new(
                                        db,
                                        lookup.clone(),
                                        module,
                                    ));
                                }
                            }
                            Ok(_) => break,
                            Err(_) => {
                                saw_unresolved_target = true;
                                break;
                            }
                        }
                    }
                    ImportSymbol::Default => {
                        if let Ok(path) = reexport.import.resolved_path.as_deref()
                            && seen_paths.insert(path.to_path_buf())
                            && let Some(module) = db.module_for_path(path)
                        {
                            stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                        }
                    }
                }
            }
            None => {
                for reexport in module.blanket_reexports.iter() {
                    match reexport.import.resolved_path.as_deref() {
                        Ok(path) => {
                            if seen_paths.insert(path.to_path_buf())
                                && let Some(module) = db.module_for_path(path)
                            {
                                stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                            }
                        }
                        Err(_) => saw_unresolved_target = true,
                    }
                }
            }
        }
    }

    if saw_unresolved_target {
        JsExportedSymbolLookup::Unknown
    } else {
        JsExportedSymbolLookup::Missing
    }
}

/// Finds JSDoc for an exported symbol by `name`, following re-exports through the db.
#[salsa::tracked(returns(ref))]
pub fn find_jsdoc_for_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Option<JsdocComment> {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![symbol];

    while let Some(symbol) = stack.pop() {
        let ModuleInfoKind::Js(module) = symbol.module(db).kind(db) else {
            continue;
        };
        match &module.exports.get(symbol.name(db).as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return match own_export {
                    JsOwnExport::Binding(binding_range) => module
                        .semantic_model
                        .as_binding_by_range(*binding_range)
                        .and_then(|binding| binding.jsdoc().cloned()),
                    JsOwnExport::Type(_) => None,
                    JsOwnExport::Namespace(reexport) => reexport
                        .export_range
                        .and_then(|range| module.semantic_model.export_jsdoc(range).cloned()),
                };
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => break,
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.module_for_path(path) {
                                    stack.push(SymbolFromModuleInfo::new(
                                        db,
                                        lookup.clone(),
                                        module,
                                    ));
                                }
                            }
                            _ => break,
                        }
                    }
                    ImportSymbol::Default => {
                        if let Ok(path) = reexport.import.resolved_path.as_deref()
                            && let Some(module) = db.module_for_path(path)
                        {
                            stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                        }
                    }
                }
            }
            None => {
                for reexport in module.blanket_reexports.iter() {
                    if let Ok(path) = reexport.import.resolved_path.as_deref()
                        && seen_paths.insert(path.to_path_buf())
                        && let Some(module) = db.module_for_path(path)
                    {
                        stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                    }
                }
            }
        }
    }

    None
}

// #endregion

// #region QUERY HELPER FUNCTIONS

// #endregion

// #region INTERNED TYPES

#[salsa::interned]
/// Generic symbol used by queries to track a generic "symbol", which can represent everything (variable name, class name, etc.)
pub struct SymbolFromModuleInfo {
    #[returns(clone)]
    name: String,

    #[returns(ref)]
    module: ModuleInfo,
}

// #endregion
