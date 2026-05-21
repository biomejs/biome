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
use crate::{CssTraversalStep, ImportTreeNode, JsExport, JsOwnExport, ModuleDb};
use biome_css_syntax::{TextRange, TextSize};
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

/// Returns CSS class steps for a JS module by traversing its direct CSS imports.
///
/// Tracked: depends on `js_module_info(db, module)` and on the CSS modules it
/// imports. If any of those change, this recomputes.
#[salsa::tracked(no_eq)]
pub fn css_classes_for_module(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<CssClassStep> {
    let module_kind = module.kind(db);
    let Some(js_info) = module_kind.as_js_module_info() else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for import_path in js_info.static_import_paths.values() {
        if let Some(path) = import_path.as_path()
            && let Some(target) = db.module_for_path(path)
            && let ModuleInfoKind::Css(css_info) = target.kind(db)
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
/// Generic symbol used by queries to track a generic "symbol", which can represent everything (variable name, class name, etc.)
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

/// Returns `true` if the given CSS `class_name` is referenced in any
/// JS or HTML file that transitively imports `css_path`.
#[salsa::tracked]
pub fn is_class_referenced_by_importers<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    class_name: SymbolName<'db>,
) -> bool {
    let importers = transitive_importers_of(db, module);

    for importer_path in &importers {
        let Some(module) = db.module_for_path(importer_path) else {
            continue;
        };
        if is_class_used_in_component_tree(db, module, class_name) {
            return true;
        }
    }
    false
}

/// Checks if a class is used in a file or any of its imported components (transitively).
#[salsa::tracked]
fn is_class_used_in_component_tree<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    class_name: SymbolName<'db>,
) -> bool {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(module);

    while let Some(module) = queue.pop_front() {
        if !visited.insert(module) {
            continue;
        }

        match module.kind(db) {
            ModuleInfoKind::Js(js_info) => {
                if js_info
                    .referenced_classes
                    .iter()
                    .any(|r| r.matches(class_name.name(db).as_str()))
                {
                    return true;
                }
                for import_path in js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                {
                    if let Some(path) = import_path.as_path()
                        && let Some(module) = db.module_for_path(path)
                    {
                        queue.push_back(module);
                    }
                }
            }
            ModuleInfoKind::Html(html_info) => {
                if html_info
                    .referenced_classes
                    .iter()
                    .any(|r| r.matches(class_name.name(db).as_str()))
                {
                    return true;
                }
            }
            ModuleInfoKind::Css(_) => {}
        }
    }

    false
}

/// Finds JSDoc for an exported symbol by `name`, following re-exports through the db.
#[salsa::tracked]
pub fn find_jsdoc_for_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    symbol_name: SymbolName<'db>,
) -> Option<JsdocComment> {
    let ModuleInfoKind::Js(module) = module.kind(db) else {
        return None;
    };
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![(module, symbol_name)];

    while let Some((module, symbol_name)) = stack.pop() {
        match &module.exports.get(symbol_name.name(db).as_str()) {
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

/// Finds the CSS file and text range where a class is defined.
#[salsa::tracked]
pub fn find_css_class_definition<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    class_name: SymbolName<'db>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = Vec::new();
    let mut visited_css = FxHashSet::default();
    // 1. Check inline style classes in HTML-like files (carry content_offset)
    if let ModuleInfoKind::Html(html_info) = module.kind(db) {
        for class_def in &html_info.style_classes {
            if class_def.name.text() == class_name.name(db) {
                result.push((
                    module.path(db).to_path_buf(),
                    class_def.range,
                    class_def.content_offset,
                ));
            }
        }
    }

    // 2. Check CSS files reachable from HTML (linked stylesheets + script imports)
    for step in traverse_import_tree_for_html_classes(db, module) {
        if &step.css_path == module.path(db) {
            continue; // Already checked inline styles above
        }
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };

        let this_result = search_css_class_transitive(db, module, class_name, &mut visited_css);
        result.extend(this_result);
    }

    // 3. Check CSS files imported by JS (e.g., `import './styles.css'` in JSX)
    for step in traverse_import_tree_for_classes(db, module) {
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };
        let this_result = search_css_class_transitive(db, module, class_name, &mut visited_css);

        result.extend(this_result);
    }

    result
}

fn search_css_class_transitive<'db>(
    db: &'db dyn ModuleDb,
    css_module: ModuleInfo,
    class_name: SymbolName<'db>,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(css_module);

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.path(db).to_path_buf()) {
            continue;
        }

        let ModuleInfoKind::Css(css_info) = current.kind(db) else {
            continue;
        };

        for (range, token) in css_info.classes.iter() {
            if token.text() == class_name.name(db) {
                result.push((current.path(db).to_path_buf(), *range, None));
            }
        }

        // Follow @import edges
        for import in css_info.imports.values() {
            if let Some(imported_path) = import.resolved_path.as_path()
                && let Some(module) = db.module_for_path(imported_path)
            {
                queue.push_back(module);
            }
        }
    }

    result
}

/// Builds a tree structure representing the import relationships for diagnostic display.
#[salsa::tracked]
pub fn build_import_tree_for_js(db: &dyn ModuleDb, module: ModuleInfo) -> Option<ImportTreeNode> {
    let mut root = ImportTreeNode {
        file_path: module.path(db).to_path_buf(),
        css_imports: Vec::new(),
        parent_components: Vec::new(),
    };

    if let Some(js_info) = module.kind(db).as_js_module_info() {
        root.css_imports = js_info
            .static_import_paths
            .values()
            .filter_map(|import_path| {
                let path = import_path.as_path()?;
                db.css_module_info_for_path(path)?;
                Some(path.to_path_buf())
            })
            .collect();
    } else {
        return None;
    }

    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());
    root.parent_components = build_parent_nodes(db, module.path(db), &mut visited);

    Some(root)
}

/// Builds a tree structure for an HTML file's import relationships (diagnostic display).
#[salsa::tracked]
pub fn build_import_tree_for_html(db: &dyn ModuleDb, module: ModuleInfo) -> Option<ImportTreeNode> {
    let html_info = module.kind(db);
    let html_info = html_info.as_html_module_info()?;

    let css_imports: Vec<_> = html_info
        .imported_stylesheets
        .iter()
        .chain(html_info.static_import_paths.values())
        .filter_map(|stylesheet_path| {
            let path = stylesheet_path.as_path()?;
            db.css_module_info_for_path(path)?;
            Some(path.to_path_buf())
        })
        .collect();

    let mut root = ImportTreeNode {
        file_path: module.path(db).to_path_buf(),
        css_imports,
        parent_components: Vec::new(),
    };

    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());
    root.parent_components = build_parent_nodes(db, module.path(db), &mut visited);

    Some(root)
}

pub(crate) fn build_parent_nodes(
    db: &dyn ModuleDb,
    current_path: &Utf8Path,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<ImportTreeNode> {
    let all_modules = db.all_modules();
    let mut parents = Vec::new();

    for (file_path, module_info) in &all_modules {
        if visited.contains(file_path.as_path()) {
            continue;
        }

        let imports_current = match module_info {
            ModuleInfoKind::Js(js_info) => js_info
                .static_import_paths
                .values()
                .chain(js_info.dynamic_import_paths.values())
                .any(|p| p.as_path() == Some(current_path)),
            ModuleInfoKind::Html(html_info) => html_info
                .imported_stylesheets
                .iter()
                .chain(html_info.static_import_paths.values())
                .chain(html_info.dynamic_import_paths.values())
                .any(|p| p.as_path() == Some(current_path)),
            ModuleInfoKind::Css(_) => false,
        };

        if imports_current {
            let css_imports: Vec<Utf8PathBuf> = match module_info {
                ModuleInfoKind::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .filter_map(|import_path| {
                        let path = import_path.as_path()?;
                        db.css_module_info_for_path(path)?;
                        Some(path.to_path_buf())
                    })
                    .collect(),
                ModuleInfoKind::Html(html_info) => html_info
                    .imported_stylesheets
                    .iter()
                    .chain(html_info.static_import_paths.values())
                    .chain(html_info.dynamic_import_paths.values())
                    .filter_map(|stylesheet_path| {
                        let path = stylesheet_path.as_path()?;
                        db.css_module_info_for_path(path)?;
                        Some(path.to_path_buf())
                    })
                    .collect(),
                ModuleInfoKind::Css(_) => Vec::new(),
            };

            let mut branch_visited = visited.clone();
            branch_visited.insert(file_path.clone());

            let parent_components = build_parent_nodes(db, file_path, &mut branch_visited);

            parents.push(ImportTreeNode {
                file_path: file_path.clone(),
                css_imports,
                parent_components,
            });
        }
    }

    parents
}
