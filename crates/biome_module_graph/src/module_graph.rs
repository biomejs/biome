//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! Module info is stored as Salsa inputs in a `WorkspaceDb`. Query and
//! traversal functions in this module accept `&dyn ModuleDb` to look up data.
pub(crate) mod fs_proxy;

use crate::css_module_info::{CssModuleInfo, CssModuleVisitor, SerializedCssModuleInfo};
use crate::html_module_info::{
    HtmlEmbeddedContent, HtmlModuleInfo, HtmlModuleVisitor, SerializedHtmlModuleInfo,
};
use crate::path_info_cache::PathInfoCache;
use crate::{
    JsModuleInfo, ModuleDiagnostic, SerializedJsModuleInfo, js_module_info::JsModuleVisitor,
};
use biome_css_syntax::AnyCssRoot;
use biome_fs::BiomePath;
use biome_html_syntax::HtmlRoot;
use biome_js_syntax::AnyJsRoot;
use biome_project_layout::ProjectLayout;
use biome_resolver::FsWithResolverProxy;
use camino::Utf8PathBuf;
pub(crate) use fs_proxy::ModuleGraphFsProxy;
use rustc_hash::FxHashSet;
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
