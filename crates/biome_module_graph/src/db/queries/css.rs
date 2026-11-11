use super::js_scc::compute_sccs;
use super::{SymbolFromModuleInfo, module_importers};
use crate::css_module_info::TailwindPosition;
use crate::css_module_info::traverse::{
    CssClassStep, CssPropertyBranch, CssPropertyTraversal, css_class_import_paths,
    provided_css_class_steps, provides_css_classes,
};
use crate::traverse::UpwardTraversalVisitor;
use crate::{
    CssPropertyDefinition, ImportTreeNode, ModuleDb, ModuleGraphGeneration, ModuleInfo,
    ModuleInfoKind, TailwindStylesheet,
};
use biome_css_syntax::{TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::sync::Arc;

// #region EXPORTED TRACKED QUERIES

/// Returns the Tailwind CSS configuration of a stylesheet, including the CSS
/// files it imports.
///
/// Each `@import` is replaced by the content of the imported file, the way
/// Tailwind CSS's `substituteAtImports` does
/// (<https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/at-import.ts>).
/// Tailwind CSS doesn't skip files it already imported, so a file imported
/// twice is inlined twice. The configuration is collected in the resulting
/// source order, where later declarations override earlier ones, as they do
/// in the CSS cascade.
///
/// Tailwind CSS fails on an import cycle once it exceeds its recursion limit.
/// Here, an import of a file that is already being inlined is skipped
/// instead. Like Tailwind CSS, `@import url(...)` isn't inlined.
#[salsa::tracked(returns(ref))]
pub fn tailwind_stylesheet(db: &dyn ModuleDb, module: ModuleInfo) -> TailwindStylesheet {
    let mut stylesheet = TailwindStylesheet::default();
    let Some(frame) = TailwindImportFrame::new(db, module) else {
        return stylesheet;
    };
    // The stack holds the files being inlined, from the stylesheet to the
    // innermost import.
    let mut stack = vec![frame];
    while let Some(frame) = stack.last_mut() {
        let ModuleInfoKind::Css(css_info) = frame.module.kind(db) else {
            stack.pop();
            continue;
        };
        let tailwind = &css_info.tailwind;
        let Some((position, imported)) = frame.imports.next() else {
            // Every import of this file has been inlined. Add the rest of its
            // own configuration, then resume the file that imported it.
            stylesheet.extend_between(tailwind, frame.inlined, tailwind.end());
            stack.pop();
            continue;
        };
        stylesheet.extend_between(tailwind, frame.inlined, position);
        frame.inlined = position;
        if stack.iter().all(|frame| frame.module != imported)
            && let Some(frame) = TailwindImportFrame::new(db, imported)
        {
            stack.push(frame);
        }
    }
    stylesheet
}

/// A CSS file that [tailwind_stylesheet] is inlining.
struct TailwindImportFrame {
    module: ModuleInfo,
    /// The CSS modules the file imports, in import order, with where each
    /// import appears. `@import "tailwindcss"` doesn't resolve to a local file:
    /// its configuration is the default one.
    imports: std::vec::IntoIter<(TailwindPosition, ModuleInfo)>,
    /// How much of the file's own configuration has been added.
    inlined: TailwindPosition,
}

impl TailwindImportFrame {
    fn new(db: &dyn ModuleDb, module: ModuleInfo) -> Option<Self> {
        let ModuleInfoKind::Css(css_info) = module.kind(db) else {
            return None;
        };
        let imports = css_info
            .tailwind_imports
            .iter()
            .filter_map(|tailwind_import| {
                let path = tailwind_import
                    .import
                    .resolve_css(db, module)
                    .path()
                    .as_path()?;
                Some((tailwind_import.position, db.module_for_path(path)?))
            })
            .collect::<Vec<_>>()
            .into_iter();
        Some(Self {
            module,
            imports,
            inlined: TailwindPosition::default(),
        })
    }
}

/// Returns CSS class steps for a JS module by traversing its direct CSS imports.
///
/// Tracked: depends on `js_module_info(db, module)` and on the CSS modules it
/// imports. If any of those change, this recomputes.
#[salsa::tracked(returns(deref))]
pub fn css_classes_for_module(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<CssClassStep> {
    let module_kind = module.kind(db);
    let Some(js_info) = module_kind.as_js_module_info() else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for import_path in js_info
        .import_paths
        .iter()
        .filter(|import| import.kind.is_static())
    {
        let resolved = import_path.resolve_js(db, module);
        if let Some(path) = resolved.path().as_path()
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
#[salsa::tracked(returns(deref))]
pub fn transitive_importers_of(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Utf8PathBuf> {
    let mut result = Vec::new();
    let mut visited: FxHashSet<Utf8PathBuf> = FxHashSet::default();
    let mut queue = VecDeque::new();
    let importers = module_importers(db, ModuleGraphGeneration::get(db));
    queue.push_back(module.path(db).to_path_buf());

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }

        for owner in importers.for_path(&current) {
            let file_path = owner.path(db);
            if file_path.as_path() == current || visited.contains(file_path) {
                continue;
            }
            match owner.kind(db) {
                ModuleInfoKind::Js(_) | ModuleInfoKind::Html(_) => {
                    result.push(file_path.to_path_buf());
                }
                ModuleInfoKind::Css(_) => {
                    queue.push_back(file_path.to_path_buf());
                }
            }
        }
    }

    result.sort_unstable();
    result.dedup();
    result
}

/// Returns CSS classes reachable from a JavaScript module and its importers.
///
/// Direct CSS imports of `module` are returned first. The query then traverses
/// JavaScript and HTML modules that transitively import it, collecting their
/// direct CSS imports and global HTML style classes. Each importer is visited at
/// most once, including when the module graph contains cycles.
#[salsa::tracked(returns(deref))]
pub fn traverse_import_tree_for_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut results = Vec::new();

    if let Some(owner) = db.module_for_path(module.path(db)) {
        let owner_kind = owner.kind(db);
        if let Some(js_info) = owner_kind.as_js_module_info() {
            for import_path in js_info
                .import_paths
                .iter()
                .filter(|import| import.kind.is_static())
            {
                let resolved = import_path.resolve_js(db, owner);
                if let Some(path) = resolved.path().as_path()
                    && let Some(css_info) = db.css_module_info_for_path(path)
                {
                    results.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }
        }
    }

    results.extend(importer_css_class_steps(db, module));
    results
}

/// Returns CSS classes reachable from an HTML module and its importers.
///
/// The result includes the module's inline classes and linked stylesheets,
/// followed by CSS classes discovered through JavaScript and HTML importers.
/// Each importer is visited at most once, including when the module graph
/// contains cycles.
#[salsa::tracked(returns(deref))]
pub fn traverse_import_tree_for_html_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut inline_steps = Vec::new();
    let mut linked_steps = Vec::new();

    if let Some(owner) = db.module_for_path(module.path(db)) {
        let owner_kind = owner.kind(db);
        if let Some(html_info) = owner_kind.as_html_module_info() {
            let all_inline_classes: IndexMap<_, _> = html_info
                .style_classes
                .iter()
                .map(|c| (c.document_range(), c.name.clone()))
                .collect();
            if !all_inline_classes.is_empty() {
                inline_steps.push(CssClassStep {
                    css_path: module.path(db).to_path_buf(),
                    css_classes: all_inline_classes,
                });
            }

            for stylesheet_path in &html_info.imported_stylesheets {
                let resolved = stylesheet_path.resolve_css(db, owner);
                if let Some(path) = resolved.path().as_path()
                    && let Some(css_info) = db.css_module_info_for_path(path)
                {
                    linked_steps.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }

            for import_path in html_info.import_paths.iter() {
                let resolved = import_path.resolve_html(db, owner);
                if let Some(path) = resolved.path().as_path()
                    && let Some(css_info) = db.css_module_info_for_path(path)
                {
                    linked_steps.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }
        }
    }

    inline_steps
        .into_iter()
        .chain(linked_steps)
        .chain(importer_css_class_steps(db, module))
        .collect()
}

/// Returns the nearest visible custom property definitions for a CSS, JavaScript, or HTML-like module.
///
/// A CSS module's local definition takes precedence over definitions reached
/// through its imports. A JavaScript module searches the CSS files it imports.
/// An HTML-like module searches embedded styles, linked stylesheets, and script
/// imports in document order. Every importer is then traversed as an independent
/// branch. A branch stops at its first visible definition. Component-scoped
/// styles do not escape an HTML-like importer.
/// Definitions reached through the same authored rule are deduplicated, while
/// definitions from separate branches remain separate. Result order is
/// unspecified.
#[salsa::tracked(returns(deref))]
pub fn css_property_definitions<'db>(
    db: &'db dyn ModuleDb,
    property: SymbolFromModuleInfo<'db>,
) -> Vec<CssPropertyDefinition> {
    let module = *property.module(db);
    let path = module.path(db);
    let name = property.name(db);
    let traversal = CssPropertyTraversal::new(db, &name);
    let definition = match module.kind(db) {
        ModuleInfoKind::Css(_) => traversal.last_property_in_css_context(path),
        ModuleInfoKind::Html(_) => traversal.last_property_in_html_context(path),
        ModuleInfoKind::Js(_) => traversal.last_property_in_js_context(path),
    };
    if let Some(definition) = definition {
        return vec![definition];
    }

    traversal
        .into_upward_iter(
            path.to_path_buf(),
            CssPropertyBranch::new(path.to_path_buf()),
        )
        .collect()
}

/// Returns `true` if the given CSS `class_name` is referenced in any
/// JS or HTML file that transitively imports `css_path`.
#[salsa::tracked(returns(copy))]
pub fn is_class_referenced_by_importers<'db>(
    db: &'db dyn ModuleDb,
    class_name: SymbolFromModuleInfo<'db>,
) -> bool {
    let importers = transitive_importers_of(db, *class_name.module(db));

    for importer_path in importers {
        let Some(module) = db.module_for_path(importer_path) else {
            continue;
        };
        if is_class_used_in_component_tree(db, module, class_name) {
            return true;
        }
    }
    false
}

/// Finds the CSS file and text range where a class is defined.
#[salsa::tracked]
pub fn find_css_class_definition<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = Vec::new();
    let module = symbol.module(db);
    let mut visited_css = FxHashSet::default();
    // 1. Check inline style classes in HTML-like files (carry content_offset)
    if let ModuleInfoKind::Html(html_info) = module.kind(db) {
        for class_def in &html_info.style_classes {
            if class_def.name.text() == symbol.name(db) {
                result.push((
                    symbol.module(db).path(db).to_path_buf(),
                    class_def.range,
                    class_def.content_offset,
                ));
            }
        }
    }

    // 2. Check CSS files reachable from HTML (linked stylesheets + script imports)
    for step in traverse_import_tree_for_html_classes(db, *module) {
        if &step.css_path == module.path(db) {
            continue; // Already checked inline styles above
        }
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };

        let this_result = search_css_class_transitive(
            db,
            SymbolFromModuleInfo::new(db, symbol.name(db), module),
            &mut visited_css,
        );
        result.extend(this_result);
    }

    // 3. Check CSS files imported by JS (e.g., `import './styles.css'` in JSX)
    for step in traverse_import_tree_for_classes(db, *module) {
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };
        let this_result = search_css_class_transitive(
            db,
            SymbolFromModuleInfo::new(db, symbol.name(db), module),
            &mut visited_css,
        );

        result.extend(this_result);
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

    let js_info = module.kind(db);
    let js_info = js_info.as_js_module_info()?;
    root.css_imports = js_info
        .import_paths
        .iter()
        .filter(|import| import.kind.is_static())
        .filter_map(|import_path| {
            let resolved = import_path.resolve_js(db, module);
            let path = resolved.path().as_path()?;
            db.css_module_info_for_path(path)?;
            Some(path.to_path_buf())
        })
        .collect();

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
        .filter_map(|stylesheet_path| {
            let resolved = stylesheet_path.resolve_css(db, module);
            let path = resolved.path().as_path()?;
            db.css_module_info_for_path(path)?;
            Some(path.to_path_buf())
        })
        .chain(html_info.import_paths.iter().filter_map(|import_path| {
            let resolved = import_path.resolve_html(db, module);
            let path = resolved.path().as_path()?;
            db.css_module_info_for_path(path)?;
            Some(path.to_path_buf())
        }))
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

// #endregion

// #region QUERY HELPER FUNCTIONS

/// Returns the CSS class steps that the importers of `module` provide to it.
///
/// A module sees the classes of every JavaScript or HTML module that reaches
/// it through imports, directly or through other importers. Each of those
/// modules contributes once, in path order. `module` doesn't contribute to its
/// own result, even when an import cycle leads back to it.
fn importer_css_class_steps(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> impl Iterator<Item = CssClassStep> + '_ {
    css_class_providers(db, ModuleGraphGeneration::get(db))
        .providers_of(module)
        .flat_map(move |provider| provided_css_class_steps(db, provider))
}

/// The modules that provide CSS classes to each module through importers.
///
/// A module reaches the modules it imports through [css_class_import_paths],
/// and those modules reach the modules they import in turn. Modules that
/// reach each other through an import cycle form one component and share one
/// provider list.
#[derive(Debug, Default, Eq, PartialEq)]
struct CssClassProviders {
    /// The indexed modules sorted by path. A module's position is its ID.
    modules: Box<[ModuleInfo]>,

    /// The component of each module.
    component_by_module: FxHashMap<ModuleInfo, u32>,

    /// The IDs, in ascending order, of the modules that provide classes and
    /// reach each component, including providers inside the component.
    providers_by_component: Box<[Arc<[u32]>]>,
}

impl CssClassProviders {
    /// Returns the providers that reach `module`, excluding `module` itself.
    fn providers_of(&self, module: ModuleInfo) -> impl Iterator<Item = ModuleInfo> + '_ {
        self.component_by_module
            .get(&module)
            .and_then(|component| self.providers_by_component.get(*component as usize))
            .into_iter()
            .flat_map(|providers| providers.iter())
            .filter_map(|id| self.modules.get(*id as usize).copied())
            .filter(move |provider| *provider != module)
    }
}

/// Returns the CSS class providers of every indexed module.
///
/// The work is linear in the number of modules and resolved imports, plus the
/// size of the provider lists. A component that has no providers of its own
/// and a single importing component shares that component's list, so a long
/// import chain stores one list.
#[salsa::tracked(returns(ref))]
fn css_class_providers(db: &dyn ModuleDb, generation: ModuleGraphGeneration) -> CssClassProviders {
    let _ = generation.value(db);

    let mut modules = Vec::new();
    db.for_each_module(&mut |module| modules.push(module));
    // Path order makes the IDs independent of the enumeration order, so an
    // unchanged graph produces an equal index and dependent queries are reused.
    modules.sort_unstable_by(|left, right| left.path(db).cmp(right.path(db)));
    let id_by_path: FxHashMap<_, _> = modules
        .iter()
        .enumerate()
        .map(|(id, module)| (module.path(db).as_path(), id as u32))
        .collect();

    let imports: Vec<_> = modules
        .iter()
        .map(|module| css_class_imports(db, *module))
        .collect();
    let edges: Vec<Vec<_>> = imports
        .iter()
        .map(|imports| {
            imports
                .imported_paths
                .iter()
                .filter_map(|path| id_by_path.get(path.as_path()).copied())
                .collect()
        })
        .collect();
    let (component_by_id, component_sizes) = compute_sccs(&edges);

    let mut own_providers = vec![Vec::new(); component_sizes.len()];
    let mut importer_components = vec![Vec::new(); component_sizes.len()];
    for (id, (imported_ids, imports)) in edges.iter().zip(&imports).enumerate() {
        let component = component_by_id[id];
        if imports.provides_classes {
            own_providers[component as usize].push(id as u32);
        }
        for imported_id in imported_ids {
            let imported_component = component_by_id[*imported_id as usize];
            if imported_component != component {
                importer_components[imported_component as usize].push(component);
            }
        }
    }

    // `compute_sccs` numbers an importing component before the components it
    // imports, so the lists of all importing components are already complete.
    let mut providers_by_component: Vec<Arc<[u32]>> = Vec::with_capacity(component_sizes.len());
    for (mut providers, mut importers) in own_providers.into_iter().zip(importer_components) {
        importers.sort_unstable();
        importers.dedup();
        let component_providers = match importers.as_slice() {
            [importer] if providers.is_empty() => {
                Arc::clone(&providers_by_component[*importer as usize])
            }
            _ => {
                for importer in &importers {
                    providers.extend(providers_by_component[*importer as usize].iter());
                }
                providers.sort_unstable();
                providers.dedup();
                Arc::from(providers)
            }
        };
        providers_by_component.push(component_providers);
    }

    CssClassProviders {
        component_by_module: modules.iter().copied().zip(component_by_id).collect(),
        modules: modules.into_boxed_slice(),
        providers_by_component: providers_by_component.into_boxed_slice(),
    }
}

/// The imports through which a module provides CSS classes.
#[derive(Debug, Default, Eq, PartialEq)]
struct CssClassImports {
    /// The paths returned by [css_class_import_paths].
    imported_paths: Box<[Utf8PathBuf]>,

    /// The result of [provides_css_classes].
    provides_classes: bool,
}

/// Returns the imports through which `module` provides CSS classes.
///
/// [css_class_providers] reads modules only through this query. An edit that
/// keeps these imports therefore keeps the provider index valid.
#[salsa::tracked(returns(ref))]
fn css_class_imports(db: &dyn ModuleDb, module: ModuleInfo) -> CssClassImports {
    CssClassImports {
        imported_paths: css_class_import_paths(db, module).into_boxed_slice(),
        provides_classes: provides_css_classes(db, module),
    }
}

/// Checks if a class is used in a file or any of its imported components (transitively).
#[salsa::tracked(returns(copy))]
fn is_class_used_in_component_tree<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    class_name: SymbolFromModuleInfo<'db>,
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
                for import_path in js_info.import_paths.iter() {
                    let resolved = import_path.resolve_js(db, module);
                    if let Some(path) = resolved.path().as_path()
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

fn search_css_class_transitive<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(*symbol.module(db));

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.path(db).to_path_buf()) {
            continue;
        }

        let ModuleInfoKind::Css(css_info) = current.kind(db) else {
            continue;
        };

        for (range, token) in css_info.classes.iter() {
            if token.text() == symbol.name(db) {
                result.push((current.path(db).to_path_buf(), *range, None));
            }
        }

        // Follow @import edges
        for import in css_info.imports.iter() {
            let resolved = import.resolve_css(db, current);
            if let Some(imported_path) = resolved.path().as_path()
                && let Some(module) = db.module_for_path(imported_path)
            {
                queue.push_back(module);
            }
        }
    }

    result
}

fn build_parent_nodes(
    db: &dyn ModuleDb,
    current_path: &Utf8Path,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<ImportTreeNode> {
    let importers = module_importers(db, ModuleGraphGeneration::get(db))
        .for_path(current_path)
        .to_vec();
    let mut parents = Vec::new();

    for owner in importers {
        let file_path = owner.path(db);
        if visited.contains(file_path) {
            continue;
        }
        let module_info = owner.kind(db);
        let css_imports: Vec<Utf8PathBuf> = match &module_info {
            ModuleInfoKind::Js(js_info) => js_info
                .import_paths
                .iter()
                .filter(|import| import.kind.is_static())
                .filter_map(|import_path| {
                    let resolved = import_path.resolve_js(db, owner);
                    let path = resolved.path().as_path()?;
                    db.css_module_info_for_path(path)?;
                    Some(path.to_path_buf())
                })
                .collect(),
            ModuleInfoKind::Html(html_info) => html_info
                .imported_stylesheets
                .iter()
                .filter_map(|stylesheet_path| {
                    let resolved = stylesheet_path.resolve_css(db, owner);
                    let path = resolved.path().as_path()?;
                    db.css_module_info_for_path(path)?;
                    Some(path.to_path_buf())
                })
                .chain(html_info.import_paths.iter().filter_map(|import_path| {
                    let resolved = import_path.resolve_html(db, owner);
                    let path = resolved.path().as_path()?;
                    db.css_module_info_for_path(path)?;
                    Some(path.to_path_buf())
                }))
                .collect(),
            ModuleInfoKind::Css(_) => continue,
        };

        let mut branch_visited = visited.clone();
        branch_visited.insert(file_path.to_path_buf());

        let parent_components = build_parent_nodes(db, file_path, &mut branch_visited);

        parents.push(ImportTreeNode {
            file_path: file_path.to_path_buf(),
            css_imports,
            parent_components,
        });
    }

    parents
}

// #endregion

// #region INTERNED TYPES

// #endregion
