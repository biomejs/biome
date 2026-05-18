//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! Module info is stored as Salsa inputs in a `ProjectDatabase`. Query and
//! traversal functions in this module accept `&dyn ModuleDb` to look up data.

// Salsa's `#[salsa::input]` macro generates `use<...>` capture syntax that
// clippy flags as redundant. We cannot suppress it on the struct itself because
// the lint fires inside the macro expansion.
#![allow(impl_trait_redundant_captures)]

pub(crate) mod fs_proxy;

use crate::css_module_info::{
    CssModuleInfo, CssModuleVisitor, ImportTreeNode, SerializedCssModuleInfo,
};
use crate::db::inputs::ModuleDb;
use crate::db::queries::{
    transitive_importers_of, traverse_import_tree_for_classes,
    traverse_import_tree_for_html_classes,
};
use crate::html_module_info::{
    HtmlEmbeddedContent, HtmlModuleInfo, HtmlModuleVisitor, SerializedHtmlModuleInfo,
};
use crate::path_info_cache::PathInfoCache;
use crate::{
    JsExport, JsModuleInfo, JsOwnExport, ModuleDiagnostic, SerializedJsModuleInfo,
    js_module_info::JsModuleVisitor,
};
use biome_css_syntax::AnyCssRoot;
use biome_fs::BiomePath;
use biome_html_syntax::HtmlRoot;
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use biome_project_layout::ProjectLayout;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
pub(crate) use fs_proxy::ModuleGraphFsProxy;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::ops::Deref;

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node",
];

// #region Resolve functions (pure — produce module info without storing it)

/// Resolves a JS/TS file into its module info.
///
/// Pure computation: takes a parsed AST + filesystem proxy, returns module info.
/// The caller is responsible for storing the result in the database.
pub fn resolve_js_module(
    root: AnyJsRoot,
    path: &BiomePath,
    fs: &dyn FsWithResolverProxy,
    project_layout: &ProjectLayout,
    semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
    path_info_cache: &PathInfoCache,
    enable_type_inference: bool,
) -> (JsModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    path_info_cache.prepopulate_directory_path_info(fs, &[path]);

    let directory = path.parent().unwrap_or(path);
    let fs_proxy = ModuleGraphFsProxy::new(fs, path_info_cache, project_layout);
    let visitor = JsModuleVisitor::new(
        root,
        path.to_path_buf(),
        directory,
        &fs_proxy,
        semantic_model,
        enable_type_inference,
    );

    let module_info = visitor.collect_info();
    let mut dependencies = ModuleDependencies::default();
    for import_path in module_info.all_import_paths() {
        if let Some(p) = import_path.as_path() {
            dependencies.insert(p.to_path_buf());
        }
    }
    let diagnostics = module_info.diagnostics().to_vec();
    (module_info, dependencies, diagnostics)
}

pub fn resolve_css_module(
    root: AnyCssRoot,
    path: &BiomePath,
    fs: &dyn FsWithResolverProxy,
    project_layout: &ProjectLayout,
    path_info_cache: &PathInfoCache,
) -> (CssModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    path_info_cache.prepopulate_directory_path_info(fs, &[path]);

    let directory = path.parent().unwrap_or(path);
    let fs_proxy = ModuleGraphFsProxy::new(fs, path_info_cache, project_layout);
    let visitor = CssModuleVisitor::new(root, directory, &fs_proxy);

    let module = visitor.visit();
    let mut dependencies = ModuleDependencies::default();
    for (_, import) in module.0.imports.deref() {
        if let Some(p) = import.resolved_path.as_path() {
            dependencies.insert(p.to_path_buf());
        }
    }
    (module, dependencies, Vec::new())
}

pub fn resolve_html_module(
    html_root: HtmlRoot,
    embedded_content: &[HtmlEmbeddedContent],
    path: &BiomePath,
    fs: &dyn FsWithResolverProxy,
    project_layout: &ProjectLayout,
    path_info_cache: &PathInfoCache,
) -> (HtmlModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    path_info_cache.prepopulate_directory_path_info(fs, &[path]);

    let directory = path.parent().unwrap_or(path);
    let fs_proxy = ModuleGraphFsProxy::new(fs, path_info_cache, project_layout);
    let visitor = HtmlModuleVisitor::new(
        html_root,
        embedded_content,
        path.to_path_buf(),
        directory,
        &fs_proxy,
    );

    let module = visitor.visit();
    let mut dependencies = ModuleDependencies::default();
    for resolved_path in &module.imported_stylesheets {
        if let Some(p) = resolved_path.as_path() {
            dependencies.insert(p.to_path_buf());
        }
    }
    for resolved_path in module
        .static_import_paths
        .values()
        .chain(module.dynamic_import_paths.values())
    {
        if let Some(p) = resolved_path.as_path() {
            dependencies.insert(p.to_path_buf());
        }
    }
    (module, dependencies, Vec::new())
}

// #endregion

// #region: Query functions (read from &dyn ModuleDb)

/// Returns `true` if the given CSS `class_name` is referenced in any
/// JS or HTML file that transitively imports `css_path`.
pub fn is_class_referenced_by_importers(
    db: &dyn ModuleDb,
    css_path: &Utf8Path,
    class_name: &str,
) -> bool {
    let Some(module) = db.module_for_path(css_path) else {
        return false;
    };
    let importers = transitive_importers_of(db, module);

    for importer_path in &importers {
        if is_class_used_in_component_tree(db, importer_path, class_name) {
            return true;
        }
    }
    false
}

/// Checks if a class is used in a file or any of its imported components (transitively).
fn is_class_used_in_component_tree(
    db: &dyn ModuleDb,
    file_path: &Utf8Path,
    class_name: &str,
) -> bool {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(file_path.to_path_buf());

    while let Some(current_path) = queue.pop_front() {
        if !visited.insert(current_path.clone()) {
            continue;
        }

        if let Some(module_info) = db.module_info_for_path(&current_path) {
            match &module_info {
                ModuleInfoKind::Js(js_info) => {
                    if js_info
                        .referenced_classes
                        .iter()
                        .any(|r| r.matches(class_name))
                    {
                        return true;
                    }
                    for import_path in js_info
                        .static_import_paths
                        .values()
                        .chain(js_info.dynamic_import_paths.values())
                    {
                        if let Some(path) = import_path.as_path() {
                            queue.push_back(path.to_path_buf());
                        }
                    }
                }
                ModuleInfoKind::Html(html_info) => {
                    if html_info
                        .referenced_classes
                        .iter()
                        .any(|r| r.matches(class_name))
                    {
                        return true;
                    }
                }
                ModuleInfoKind::Css(_) => {}
            }
        }
    }

    false
}

/// Finds the CSS file and text range where a class is defined.
pub fn find_css_class_definition(
    db: &dyn ModuleDb,
    path: &Utf8Path,
    class_name: &str,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = Vec::new();
    let Some(module) = db.module_for_path(path) else {
        return result;
    };
    let mut visited_css = FxHashSet::default();
    if let Some(html_info) = db.html_module_info_for_path(path) {
        for class_def in &html_info.style_classes {
            if class_def.name.text() == class_name {
                result.push((
                    path.to_path_buf(),
                    class_def.range,
                    class_def.content_offset,
                ));
            }
        }
    }

    for step in traverse_import_tree_for_html_classes(db, module) {
        if step.css_path == path {
            continue;
        }

        search_css_class_transitive(
            db,
            &step.css_path,
            class_name,
            &mut result,
            &mut visited_css,
        );
    }

    for step in traverse_import_tree_for_classes(db, module) {
        search_css_class_transitive(
            db,
            &step.css_path,
            class_name,
            &mut result,
            &mut visited_css,
        );
    }

    result
}

fn search_css_class_transitive(
    db: &dyn ModuleDb,
    css_path: &Utf8Path,
    class_name: &str,
    result: &mut Vec<(Utf8PathBuf, TextRange, Option<TextSize>)>,
    visited: &mut FxHashSet<Utf8PathBuf>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(css_path.to_path_buf());

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }

        let Some(css_info) = db.css_module_info_for_path(&current) else {
            continue;
        };

        for (range, token) in css_info.classes.iter() {
            if token.text() == class_name {
                result.push((current.clone(), *range, None));
            }
        }

        for import in css_info.imports.values() {
            if let Some(imported_path) = import.resolved_path.as_path() {
                queue.push_back(imported_path.to_path_buf());
            }
        }
    }
}

/// Builds a tree structure representing the import relationships for diagnostic display.
pub fn build_import_tree(db: &dyn ModuleDb, js_path: &Utf8Path) -> Option<ImportTreeNode> {
    let mut root = ImportTreeNode {
        file_path: js_path.to_path_buf(),
        css_imports: Vec::new(),
        parent_components: Vec::new(),
    };

    if let Some(js_info) = db.js_module_info_for_path(js_path) {
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
    visited.insert(js_path.to_path_buf());
    root.parent_components = build_parent_nodes(db, js_path, &mut visited);

    Some(root)
}

fn build_parent_nodes(
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

/// Builds a tree structure for an HTML file's import relationships (diagnostic display).
pub fn build_import_tree_for_html(
    db: &dyn ModuleDb,
    html_path: &Utf8Path,
) -> Option<ImportTreeNode> {
    let html_info = db.html_module_info_for_path(html_path)?;

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
        file_path: html_path.to_path_buf(),
        css_imports,
        parent_components: Vec::new(),
    };

    let mut visited = FxHashSet::default();
    visited.insert(html_path.to_path_buf());
    root.parent_components = build_parent_nodes(db, html_path, &mut visited);

    Some(root)
}

/// Follows re-exports across modules to find the original definition of a symbol.
pub fn find_exported_symbol(
    db: &dyn ModuleDb,
    module: &JsModuleInfo,
    symbol_name: &str,
) -> Option<JsOwnExport> {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![(module.clone(), symbol_name.to_string())];

    while let Some((module, symbol_name)) = stack.pop() {
        match &module.exports.get(symbol_name.as_str()) {
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
                                    stack.push((module, lookup));
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
                        stack.push((module, symbol_name.clone()));
                    }
                }
            }
        }
    }

    None
}

pub fn find_jsdoc_for_exported_symbol(
    db: &dyn ModuleDb,
    module: &JsModuleInfo,
    symbol_name: &str,
) -> Option<JsdocComment> {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![(module.clone(), symbol_name.to_string())];

    while let Some((module, symbol_name)) = stack.pop() {
        match &module.exports.get(symbol_name.as_str()) {
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
                                    stack.push((module, lookup));
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
                        stack.push((module, symbol_name.clone()));
                    }
                }
            }
        }
    }

    None
}

// #endregion

// #region: Types (Salsa input, enums, serialization, dependencies)
#[salsa::input]
#[derive(Debug)]
pub struct ModuleInfo {
    #[returns(ref)]
    pub path: Utf8PathBuf,

    #[no_eq]
    pub kind: ModuleInfoKind,
}

#[derive(Debug, Clone)]
pub enum ModuleInfoKind {
    Js(JsModuleInfo),
    Css(CssModuleInfo),
    Html(HtmlModuleInfo),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SerializedModuleInfo {
    Js(SerializedJsModuleInfo),
    Css(SerializedCssModuleInfo),
    Html(SerializedHtmlModuleInfo),
}

impl SerializedModuleInfo {
    pub fn as_js_module_info(&self) -> Option<&SerializedJsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&SerializedCssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_html_module_info(&self) -> Option<&SerializedHtmlModuleInfo> {
        match self {
            Self::Html(module) => Some(module),
            _ => None,
        }
    }
}

impl From<JsModuleInfo> for ModuleInfoKind {
    fn from(info: JsModuleInfo) -> Self {
        Self::Js(info)
    }
}

impl From<CssModuleInfo> for ModuleInfoKind {
    fn from(info: CssModuleInfo) -> Self {
        Self::Css(info)
    }
}

impl From<HtmlModuleInfo> for ModuleInfoKind {
    fn from(info: HtmlModuleInfo) -> Self {
        Self::Html(info)
    }
}

impl ModuleInfoKind {
    pub fn dump(&self) -> SerializedModuleInfo {
        match self {
            Self::Js(module) => SerializedModuleInfo::Js(module.dump()),
            Self::Css(module) => SerializedModuleInfo::Css(module.dump()),
            Self::Html(module) => SerializedModuleInfo::Html(module.dump()),
        }
    }

    pub fn as_js_module_info(&self) -> Option<&JsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&CssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_html_module_info(&self) -> Option<&HtmlModuleInfo> {
        match self {
            Self::Html(module) => Some(module),
            _ => None,
        }
    }
}

/// Represents all the files that are imported/depended on by a module.
#[derive(Debug, Default)]
pub struct ModuleDependencies(FxHashSet<Utf8PathBuf>);

impl ModuleDependencies {
    pub fn insert(&mut self, dependency_path: Utf8PathBuf) {
        self.0.insert(dependency_path);
    }
}

impl AsRef<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn as_ref(&self) -> &FxHashSet<Utf8PathBuf> {
        &self.0
    }
}

impl Deref for ModuleDependencies {
    type Target = FxHashSet<Utf8PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn from(dependencies: FxHashSet<Utf8PathBuf>) -> Self {
        Self(dependencies)
    }
}

impl FromIterator<Utf8PathBuf> for ModuleDependencies {
    fn from_iter<T: IntoIterator<Item = Utf8PathBuf>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for ModuleDependencies {
    type Item = Utf8PathBuf;

    type IntoIter = <FxHashSet<Utf8PathBuf> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

//#endregion
