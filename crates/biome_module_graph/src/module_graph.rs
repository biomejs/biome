//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! Module info is stored as Salsa inputs in a `WorkspaceDb`. Query and
//! traversal functions in this module accept `&dyn ModuleDb` to look up data.
use crate::css_module_info::{CssModuleInfo, CssModuleVisitor, SerializedCssModuleInfo};
use crate::db::queries::{css_dependencies, html_dependencies, js_dependencies};
use crate::html_module_info::{
    HtmlModuleInfo, HtmlModuleVisitor, PreparedHtmlModule, SerializedHtmlModuleInfo,
    prepare_html_module,
};
use crate::{
    JsModuleInfo, ModuleDb, ModuleDiagnostic, SerializedJsModuleInfo, TypeInferenceMode,
    js_module_info::JsModuleVisitor,
};
use biome_css_syntax::AnyCssRoot;
use biome_fs::BiomePath;
use biome_js_syntax::AnyJsRoot;
use biome_resolver::ResolverDb;
use camino::Utf8PathBuf;
use rustc_hash::FxHashSet;
use std::ops::Deref;

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node",
];

// #region Resolve functions (pure — produce module info without storing it)

/// Resolves a JS/TS file into its module info.
///
/// Collects the module info from the AST and resolves its imports through the
/// database, which records the path info observed by the resolver.
/// The caller is responsible for storing the module info in the database.
pub fn resolve_js_module(
    db: &dyn ResolverDb,
    root: AnyJsRoot,
    path: &BiomePath,
    semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
    enable_type_inference: bool,
) -> (JsModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    let inference_mode = if enable_type_inference {
        TypeInferenceMode::RawTypesOnly
    } else {
        TypeInferenceMode::Disabled
    };

    resolve_js_module_with_inference_mode(db, root, path, semantic_model, inference_mode)
}

pub fn resolve_js_module_with_inference_mode(
    db: &dyn ResolverDb,
    root: AnyJsRoot,
    path: &BiomePath,
    semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
    inference_mode: TypeInferenceMode,
) -> (JsModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    let visitor = JsModuleVisitor::new(root, path.to_path_buf(), semantic_model, inference_mode);
    let module_info = visitor.collect_info();
    let dependencies = js_dependencies(db, path, &module_info);
    let diagnostics = module_info.diagnostics().to_vec();
    (module_info, dependencies, diagnostics)
}

pub fn resolve_css_module(
    db: &dyn ResolverDb,
    root: AnyCssRoot,
    path: &BiomePath,
) -> (CssModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    let module = CssModuleVisitor::new(root).visit();
    let dependencies = css_dependencies(db, path, &module);
    (module, dependencies, Vec::new())
}

pub fn resolve_html_module(
    db: &dyn ModuleDb,
    path: &BiomePath,
) -> Option<(HtmlModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>)> {
    let parsed_source = db.parsed_source_for_path(path)?;
    let prepared = prepare_html_module(db, parsed_source);
    Some(resolve_prepared_html_module(db, prepared, path))
}

pub fn resolve_prepared_html_module(
    db: &dyn ResolverDb,
    prepared: PreparedHtmlModule,
    path: &BiomePath,
) -> (HtmlModuleInfo, ModuleDependencies, Vec<ModuleDiagnostic>) {
    let module = HtmlModuleVisitor::new(prepared, path.to_path_buf()).visit();
    let dependencies = html_dependencies(db, path, &module);
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

impl ModuleInfo {
    pub fn dump(self, db: &dyn ModuleDb) -> SerializedModuleInfo {
        match self.kind(db) {
            ModuleInfoKind::Js(module) => SerializedModuleInfo::Js(module.dump(db, self)),
            ModuleInfoKind::Css(module) => SerializedModuleInfo::Css(module.dump()),
            ModuleInfoKind::Html(module) => SerializedModuleInfo::Html(module.dump()),
        }
    }
}

impl ModuleInfoKind {
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

impl Extend<Utf8PathBuf> for ModuleDependencies {
    fn extend<T: IntoIterator<Item = Utf8PathBuf>>(&mut self, iter: T) {
        self.0.extend(iter);
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
