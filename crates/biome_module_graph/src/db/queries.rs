//! This module represents the database queries used by the module graph.
//!
//! The queries are defined in terms of `ModuleInfo` inputs.
//!
//! The queries are tracked so that Salsa can invalidate them when the inputs
//! change.
//!
//! The queries are also interned, so that Salsa can reuse the same computation
//! when the inputs are the same.

use crate::css_module_info::traverse::{CssClassStep, ImportTreeTraversal};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{CssModuleInfo, CssTraversalStep, JsExport, JsModuleInfo, JsOwnExport, ModuleDb};
use biome_js_type_info::ImportSymbol;
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

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

/// Returns all files that transitively import `path` (through CSS `@import`
/// chains and HTML `<link>` references).
///
/// The returned set includes only JS/HTML files (potential class consumers),
/// not intermediate CSS files.
#[salsa::tracked]
pub fn transitive_importers_of(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Utf8PathBuf> {
    let mut result = Vec::new();
    let mut visited: FxHashSet<Utf8PathBuf> = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(module.path(db).to_path_buf());

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }

        db.for_each_module(&mut |file_path, module_info| {
            if file_path == current.as_path() {
                return;
            }
            let imports_current = match module_info {
                ModuleInfoKind::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                    .any(|p| p.as_path() == Some(current.as_path())),
                ModuleInfoKind::Css(css_info) => css_info
                    .imports
                    .values()
                    .any(|p| p.resolved_path.as_path() == Some(current.as_path())),
                ModuleInfoKind::Html(html_info) => {
                    html_info
                        .imported_stylesheets
                        .iter()
                        .any(|p| p.as_path() == Some(current.as_path()))
                        || html_info
                            .static_import_paths
                            .values()
                            .any(|p| p.as_path() == Some(current.as_path()))
                        || html_info
                            .dynamic_import_paths
                            .values()
                            .any(|p| p.as_path() == Some(current.as_path()))
                }
            };

            if imports_current && !visited.contains(file_path) {
                match module_info {
                    ModuleInfoKind::Js(_) | ModuleInfoKind::Html(_) => {
                        result.push(file_path.to_path_buf());
                    }
                    ModuleInfoKind::Css(_) => {
                        queue.push_back(file_path.to_path_buf());
                    }
                }
            }
        });
    }

    result
}

/// Returns CSS class steps for the given JS file by traversing its imports.
#[salsa::tracked]
pub fn traverse_import_tree_for_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut results = Vec::new();

    if let Some(js_info) = db.js_module_info_for_path(module.path(db)) {
        for import_path in js_info.static_import_paths.values() {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                results.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }
    }

    let stack = vec![module.path(db).to_path_buf()];
    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());

    let traversal = ImportTreeTraversal {
        module_database: db,
        stack,
        visited,
        current_css_iter: None,
    };
    results.extend(traversal);
    results
}

/// Returns CSS class steps for the given HTML file.
#[salsa::tracked]
pub fn traverse_import_tree_for_html_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut inline_steps = Vec::new();
    let mut linked_steps = Vec::new();

    if let Some(html_info) = db.html_module_info_for_path(module.path(db)) {
        let all_inline_classes: IndexMap<_, _> = html_info
            .style_classes
            .iter()
            .map(|c| (c.range, c.name.clone()))
            .collect();
        if !all_inline_classes.is_empty() {
            inline_steps.push(CssClassStep {
                css_path: module.path(db).to_path_buf(),
                css_classes: all_inline_classes,
            });
        }

        for stylesheet_path in &html_info.imported_stylesheets {
            if let Some(path) = stylesheet_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                linked_steps.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }

        for import_path in html_info.static_import_paths.values() {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                linked_steps.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }

        for import_path in html_info
            .static_import_paths
            .values()
            .chain(html_info.dynamic_import_paths.values())
        {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                linked_steps.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }
    }

    let stack = vec![module.path(db).to_path_buf()];
    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());

    inline_steps
        .into_iter()
        .chain(linked_steps)
        .chain(ImportTreeTraversal {
            module_database: db,
            stack,
            visited,
            current_css_iter: None,
        })
        .collect()
}

/// Collects all CSS classes available through the import tree of the given JS file.
#[salsa::tracked]
pub fn collect_available_classes_for_js_file(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> (FxHashSet<String>, Vec<CssTraversalStep>) {
    let mut available_classes = FxHashSet::default();
    let mut traversal_path = Vec::new();
    let mut visited = FxHashSet::default();
    let all_modules = db.all_modules();

    if let Some(js_info) = db.js_module_info_for_path(module.path(db)) {
        for import_path in js_info
            .static_import_paths
            .values()
            .chain(js_info.dynamic_import_paths.values())
        {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                for class in css_info.classes.values() {
                    let class_name = class.text().to_string();
                    available_classes.insert(class_name);
                }
                traversal_path.push(CssTraversalStep {
                    css_path: path.to_path_buf(),
                    importer_path: module.path(db).to_path_buf(),
                    component_chain: vec![module.path(db).to_path_buf()],
                    is_direct: true,
                });
            }
        }
    }

    let mut queue: VecDeque<_> = VecDeque::new();
    queue.push_back((
        module.path(db).to_path_buf(),
        vec![module.path(db).to_path_buf()],
    ));
    visited.insert(module.path(db).to_path_buf());

    while let Some((current_path, current_chain)) = queue.pop_front() {
        for (file_path, module_info) in &all_modules {
            if visited.contains(file_path.as_path()) {
                continue;
            }

            let imports_current = match module_info {
                ModuleInfoKind::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                    .any(|p| p.as_path() == Some(current_path.as_path())),
                ModuleInfoKind::Html(html_info) => html_info
                    .imported_stylesheets
                    .iter()
                    .chain(html_info.static_import_paths.values())
                    .chain(html_info.dynamic_import_paths.values())
                    .any(|p| p.as_path() == Some(current_path.as_path())),
                ModuleInfoKind::Css(_) => false,
            };

            if imports_current {
                visited.insert(file_path.clone());

                match module_info {
                    ModuleInfoKind::Js(js_info) => {
                        let mut new_chain = current_chain.clone();
                        new_chain.push(file_path.clone());

                        for import_path in js_info
                            .static_import_paths
                            .values()
                            .chain(js_info.dynamic_import_paths.values())
                        {
                            if let Some(path) = import_path.as_path()
                                && let Some(css_info) = db.css_module_info_for_path(path)
                            {
                                for class in css_info.classes.values() {
                                    let class_name = class.text().to_string();
                                    available_classes.insert(class_name);
                                }

                                traversal_path.push(CssTraversalStep {
                                    css_path: path.to_path_buf(),
                                    importer_path: file_path.clone(),
                                    component_chain: new_chain.clone(),
                                    is_direct: false,
                                });
                            }
                        }
                        queue.push_back((file_path.clone(), new_chain));
                    }
                    ModuleInfoKind::Html(html_info) => {
                        let mut new_chain = current_chain.clone();
                        new_chain.push(file_path.clone());

                        for stylesheet_path in html_info
                            .imported_stylesheets
                            .iter()
                            .chain(html_info.static_import_paths.values())
                            .chain(html_info.dynamic_import_paths.values())
                        {
                            if let Some(path) = stylesheet_path.as_path()
                                && let Some(css_info) = db.css_module_info_for_path(path)
                            {
                                for class in css_info.classes.values() {
                                    let class_name = class.text().to_string();
                                    available_classes.insert(class_name);
                                }
                                traversal_path.push(CssTraversalStep {
                                    css_path: path.to_path_buf(),
                                    importer_path: file_path.clone(),
                                    component_chain: new_chain.clone(),
                                    is_direct: false,
                                });
                            }
                        }
                        queue.push_back((file_path.clone(), new_chain));
                    }
                    ModuleInfoKind::Css(_) => {}
                }
            }
        }
    }

    (available_classes, traversal_path)
}

#[salsa::interned]
pub struct SymbolName {
    #[returns(ref)]
    name: String,
}

/// Finds the default exported symbol.
#[salsa::tracked]
pub fn find_js_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    symbol_name: SymbolName<'db>,
) -> Option<JsOwnExport> {
    let ModuleInfoKind::Js(module) = module.kind(db) else {
        return None;
    };

    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![(module.clone(), symbol_name)];

    while let Some((module, symbol_name)) = stack.pop() {
        match &module.exports.get(symbol_name.name(db).as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return Some(own_export.clone());
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => break,
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.js_module_info_for_path(path) {
                                    stack.push((module, SymbolName::new(db, lookup.clone())));
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
                        stack.push((module, symbol_name));
                    }
                }
            }
        }
    }

    None
}
