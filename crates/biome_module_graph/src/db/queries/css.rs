use super::SymbolFromModuleInfo;
use crate::css_module_info::TailwindPosition;
use crate::css_module_info::traverse::{
    CssClassStep, CssClassTraversal, CssPropertyBranch, CssPropertyTraversal, css_class_step,
};
use crate::traverse::UpwardTraversalVisitor;
use crate::{
    CssClassReference, CssPropertyDefinition, FrameworkScope, ImportTreeNode, ModuleDb, ModuleInfo,
    ModuleInfoKind, TailwindStylesheet,
};
use biome_css_syntax::{TextRange, TextSize};
use biome_fs::is_node_modules_path;
use biome_languages::DocumentFileSource;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
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
    let mut modules = db.all_modules().into_iter().collect::<Vec<_>>();
    modules.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    queue.push_back(module.path(db).to_path_buf());

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }

        for (file_path, _) in &modules {
            if file_path == current.as_path() {
                continue;
            }
            let Some(owner) = db.module_for_path(file_path) else {
                continue;
            };
            let module_info = owner.kind(db);
            let imports_current =
                match &module_info {
                    ModuleInfoKind::Js(js_info) => js_info.import_paths.iter().any(|p| {
                        p.resolve_js(db, owner).path().as_path() == Some(current.as_path())
                    }),
                    ModuleInfoKind::Css(css_info) => css_info.imports.iter().any(|p| {
                        p.resolve_css(db, owner).path().as_path() == Some(current.as_path())
                    }),
                    ModuleInfoKind::Html(html_info) => {
                        html_info.imported_stylesheets.iter().any(|p| {
                            p.resolve_css(db, owner).path().as_path() == Some(current.as_path())
                        }) || html_info.import_paths.iter().any(|p| {
                            p.resolve_html(db, owner).path().as_path() == Some(current.as_path())
                        })
                    }
                };

            if imports_current && !visited.contains(file_path.as_path()) {
                match &module_info {
                    ModuleInfoKind::Js(_) | ModuleInfoKind::Html(_) => {
                        result.push(file_path.to_path_buf());
                    }
                    ModuleInfoKind::Css(_) => {
                        queue.push_back(file_path.to_path_buf());
                    }
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

    let start = module.path(db);
    let traversal = CssClassTraversal::new(db, start).into_upward_iter(start.to_path_buf(), ());
    results.extend(traversal);
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
                .map(|c| (c.range, c.name.clone()))
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
        .chain(
            CssClassTraversal::new(db, module.path(db))
                .into_upward_iter(module.path(db).to_path_buf(), ()),
        )
        .collect()
}

/// Returns the classes that the frameworks of the package of `module` make
/// available to every page: the classes of the stylesheets that they apply,
/// and of the stylesheets that these import, then the global styles of their
/// components.
///
/// Unlike [traverse_import_tree_for_classes] and
/// [traverse_import_tree_for_html_classes], the result doesn't depend on the
/// imports and importers of `module`.
pub fn framework_class_steps(db: &dyn ModuleDb, module: ModuleInfo) -> &[CssClassStep] {
    let Some(scope) = db.framework_scope(module.path(db)) else {
        return &[];
    };
    framework_package_class_steps(db, FrameworkPackage::new(db, scope.package_path.clone()))
}

/// Returns the classes that the frameworks of `package` make available to
/// every page. See [framework_class_steps].
#[salsa::tracked(returns(deref))]
fn framework_package_class_steps<'db>(
    db: &'db dyn ModuleDb,
    package: FrameworkPackage<'db>,
) -> Vec<CssClassStep> {
    let Some(scope) = db.framework_scope(package.path(db)) else {
        return Vec::new();
    };
    let mut stylesheets: Vec<_> = framework_package_stylesheets(db, package).iter().collect();
    stylesheets.sort_unstable();
    let components = scope.components.iter().filter_map(|component| {
        let info = db.html_module_info_for_path(component)?;
        Some(CssClassStep {
            css_path: component.clone(),
            css_classes: info.get_global_styles(),
        })
    });
    stylesheets
        .into_iter()
        .filter_map(|stylesheet| css_class_step(db, stylesheet))
        .chain(components)
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
///
/// When no branch finds a definition, the definitions that frameworks make
/// visible to every page are returned: the frameworks of the package that
/// contains the module, and of the packages that apply the module, a
/// stylesheet, to every page.
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

    let definitions: Vec<_> = traversal
        .into_upward_iter(
            path.to_path_buf(),
            CssPropertyBranch::new(path.to_path_buf()),
        )
        .collect();
    if !definitions.is_empty() {
        return definitions;
    }
    framework_property_definitions(db, module, &name)
}

/// Returns whether the stylesheets that frameworks apply to the pages where
/// `module` applies can't all be known: the frameworks of the package that
/// contains `module`, or of a package that applies `module`, a stylesheet, to
/// every page.
///
/// Rules that report a missing style skip such a module.
#[salsa::tracked(returns(copy))]
pub fn has_unknown_framework_styles(db: &dyn ModuleDb, module: ModuleInfo) -> bool {
    db.framework_scope(module.path(db))
        .is_some_and(|scope| scope.has_unknown_styles)
        || framework_scopes_applying(db, module)
            .iter()
            .any(|scope| scope.has_unknown_styles)
}

/// Returns `true` if the given CSS `class_name` is referenced in any
/// JS or HTML file that transitively imports `css_path`.
///
/// A class of a stylesheet that the frameworks of a package apply to every
/// page is also referenced when any file of that package, or any component of
/// the frameworks, references it. Such a class is assumed to be referenced
/// when some pages of that package can't be read. Every class of a package is
/// assumed to be referenced when the stylesheets that its frameworks apply to
/// every page can't all be known.
#[salsa::tracked(returns(copy))]
pub fn is_class_referenced_by_importers<'db>(
    db: &'db dyn ModuleDb,
    class_name: SymbolFromModuleInfo<'db>,
) -> bool {
    let module = *class_name.module(db);
    if has_unknown_framework_styles(db, module) {
        return true;
    }
    for scope in framework_scopes_applying(db, module) {
        let package = FrameworkPackage::new(db, scope.package_path.clone());
        match framework_package_class_usages(db, package) {
            None => return true,
            Some(usages) if usages.contains(&class_name.name(db)) => return true,
            Some(_) => {}
        }
    }

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

    // 4. Check the CSS files that frameworks apply to every page
    for step in framework_class_steps(db, *module) {
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

/// Returns the classes that the pages of `package` use, or `None` when some
/// pages can't be read.
///
/// Every JavaScript and HTML-like file of the package can use a class of a
/// stylesheet that its frameworks apply to every page, and so can the
/// components of the frameworks: their class attributes and the classes of
/// their local styles count as uses. A file belongs to the nearest package
/// that contains it. The pages can't be read when the frameworks report it, or
/// when the package has an HTML-like file whose markup Biome didn't read, such
/// as an Astro file indexed without full HTML support.
#[salsa::tracked(returns(ref))]
fn framework_package_class_usages<'db>(
    db: &'db dyn ModuleDb,
    package: FrameworkPackage<'db>,
) -> Option<FxHashSet<String>> {
    let Some(scope) = db.framework_scope(package.path(db)) else {
        return Some(FxHashSet::default());
    };
    if scope.has_unknown_class_usages {
        return None;
    }

    let mut usages = FxHashSet::default();
    let mut has_unread_markup = false;
    db.for_each_module(&mut |module| {
        let path = module.path(db);
        // Dependencies aren't pages of the package. The components that the
        // frameworks render are counted below.
        let is_in_package = !is_node_modules_path(path)
            && db
                .framework_scope(path)
                .is_some_and(|nearest| nearest.package_path == scope.package_path);
        if !is_in_package {
            return;
        }
        match module.kind(db) {
            ModuleInfoKind::Js(_)
                if DocumentFileSource::from_well_known(path, true).is_html_like() =>
            {
                has_unread_markup = true;
            }
            ModuleInfoKind::Js(info) => usages.extend(class_names(&info.referenced_classes)),
            ModuleInfoKind::Html(info) => usages.extend(class_names(&info.referenced_classes)),
            ModuleInfoKind::Css(_) => {}
        }
    });
    if has_unread_markup {
        return None;
    }
    for component in &scope.components {
        let Some(info) = db.html_module_info_for_path(component) else {
            continue;
        };
        usages.extend(class_names(&info.referenced_classes));
        usages.extend(
            info.style_classes
                .iter()
                .filter(|class| !class.applicability.is_global())
                .map(|class| class.name.text().to_string()),
        );
    }
    Some(usages)
}

/// Returns the class names of the class attributes in `references`.
fn class_names(references: &[CssClassReference]) -> impl Iterator<Item = String> + '_ {
    references.iter().flat_map(|reference| {
        reference
            .token
            .text()
            .split_ascii_whitespace()
            .map(str::to_string)
    })
}

/// Returns the stylesheets that the frameworks of `package` apply to every
/// page. See [framework_stylesheets].
#[salsa::tracked(returns(ref))]
fn framework_package_stylesheets<'db>(
    db: &'db dyn ModuleDb,
    package: FrameworkPackage<'db>,
) -> FxHashSet<Utf8PathBuf> {
    db.framework_scope(package.path(db))
        .map_or_else(FxHashSet::default, |scope| {
            framework_stylesheets(db, &scope)
        })
}

/// Returns the stylesheets that the frameworks of a package apply to every
/// page: the stylesheets of `scope`, the stylesheets that the components of
/// `scope` import globally, and the stylesheets that these import.
fn framework_stylesheets(db: &dyn ModuleDb, scope: &FrameworkScope) -> FxHashSet<Utf8PathBuf> {
    let mut stylesheets = FxHashSet::default();
    let mut queue: VecDeque<_> = scope.stylesheets.iter().cloned().collect();
    for component in &scope.components {
        queue.extend(global_imports_of_component(db, component));
    }
    while let Some(path) = queue.pop_front() {
        if stylesheets.contains(&path) {
            continue;
        }
        let Some(module) = db.module_for_path(&path) else {
            continue;
        };
        let ModuleInfoKind::Css(info) = module.kind(db) else {
            continue;
        };
        queue.extend(info.imports.iter().filter_map(|import| {
            let resolved = import.resolve_css(db, module);
            Some(resolved.path().as_path()?.to_path_buf())
        }));
        stylesheets.insert(path);
    }
    stylesheets
}

/// Returns the paths that the HTML-like component at `path` imports for
/// every page, with `<link>` elements, global `<style>` blocks and script
/// imports.
fn global_imports_of_component(db: &dyn ModuleDb, path: &Utf8Path) -> Vec<Utf8PathBuf> {
    let Some(module) = db.module_for_path(path) else {
        return Vec::new();
    };
    let ModuleInfoKind::Html(info) = module.kind(db) else {
        return Vec::new();
    };
    let stylesheets = info
        .imported_stylesheets
        .iter()
        .filter(|import| import.applicability.is_global())
        .map(|import| import.resolve_css(db, module));
    let scripts = info
        .import_paths
        .iter()
        .filter(|import| import.applicability.is_global())
        .map(|import| import.resolve_html(db, module));
    stylesheets
        .chain(scripts)
        .filter_map(|resolved| Some(resolved.path().as_path()?.to_path_buf()))
        .collect()
}

/// Returns the scopes of the packages whose frameworks apply the stylesheet
/// `module` to every page, directly or through a stylesheet that imports it.
///
/// The stylesheet can be outside the folders of these packages, for example
/// in a sibling package of a monorepo. A module that isn't a stylesheet has
/// no such package.
#[salsa::tracked(returns(deref))]
fn framework_scopes_applying(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Arc<FrameworkScope>> {
    if !matches!(module.kind(db), ModuleInfoKind::Css(_)) {
        return Vec::new();
    }
    let path = module.path(db);
    db.framework_scopes()
        .into_iter()
        .filter(|scope| {
            let package = FrameworkPackage::new(db, scope.package_path.clone());
            framework_package_stylesheets(db, package).contains(path)
        })
        .collect()
}

/// Returns the definitions of the custom property `name` that frameworks
/// make visible to every page where `module` applies.
///
/// These are the frameworks of the package that contains `module`, and of the
/// packages that apply `module`, a stylesheet, to every page. See
/// [framework_package_property_definitions].
fn framework_property_definitions(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    name: &str,
) -> Vec<CssPropertyDefinition> {
    let nearest = db.framework_scope(module.path(db));
    let mut packages = FxHashSet::default();
    let mut definitions = FxHashSet::default();
    nearest
        .iter()
        .chain(framework_scopes_applying(db, module))
        .map(|scope| FrameworkPackage::new(db, scope.package_path.clone()))
        .filter(|package| packages.insert(*package))
        .flat_map(|package| framework_package_property_definitions(db, package, name.to_string()))
        .filter(|definition| definitions.insert(*definition))
        .cloned()
        .collect()
}

/// Returns the definitions of the custom property `name` that the frameworks
/// of `package` make visible to every page: the last definition of each
/// stylesheet and component that they apply, and the definitions of their
/// configuration.
#[salsa::tracked(returns(deref))]
fn framework_package_property_definitions<'db>(
    db: &'db dyn ModuleDb,
    package: FrameworkPackage<'db>,
    name: String,
) -> Vec<CssPropertyDefinition> {
    let Some(scope) = db.framework_scope(package.path(db)) else {
        return Vec::new();
    };
    let traversal = CssPropertyTraversal::new(db, &name);
    let mut definitions: Vec<_> = scope
        .stylesheets
        .iter()
        .chain(&scope.components)
        .filter_map(|path| traversal.last_property_in_global_context(path))
        .chain(
            scope
                .custom_properties
                .iter()
                .filter(|(property, _)| property.text() == name)
                .map(|(_, definition)| definition.clone()),
        )
        .collect();
    let mut seen = FxHashSet::default();
    definitions.retain(|definition| seen.insert(definition.clone()));
    definitions
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
    let mut all_modules = db.all_modules().into_iter().collect::<Vec<_>>();
    all_modules.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    let mut parents = Vec::new();

    for (file_path, _) in &all_modules {
        if visited.contains(file_path.as_path()) {
            continue;
        }
        let Some(owner) = db.module_for_path(file_path) else {
            continue;
        };
        let module_info = owner.kind(db);

        let imports_current = match &module_info {
            ModuleInfoKind::Js(js_info) => js_info
                .import_paths
                .iter()
                .any(|p| p.resolve_js(db, owner).path().as_path() == Some(current_path)),
            ModuleInfoKind::Html(html_info) => {
                html_info
                    .imported_stylesheets
                    .iter()
                    .any(|p| p.resolve_css(db, owner).path().as_path() == Some(current_path))
                    || html_info
                        .import_paths
                        .iter()
                        .any(|p| p.resolve_html(db, owner).path().as_path() == Some(current_path))
            }
            ModuleInfoKind::Css(_) => false,
        };

        if imports_current {
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

// #endregion

// #region INTERNED TYPES

/// The folder of a package, as the key of the queries over what its
/// frameworks inject, so that they run once for all the files of the package.
#[salsa::interned]
struct FrameworkPackage {
    #[returns(ref)]
    path: Utf8PathBuf,
}

// #endregion
