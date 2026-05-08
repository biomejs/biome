#![deny(clippy::use_self)]

mod css_module_info;
mod db;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
mod js_module_info;
mod module_graph;
mod path_info_cache;

pub use crate::css_module_info::{
    CssClassReference, CssClassStep, CssImport, CssImports, CssModuleInfo, CssTraversalStep,
    ImportTreeDisplay, ImportTreeNode,
};
pub use crate::html_module_info::{HtmlEmbeddedContent, HtmlModuleInfo, SerializedHtmlModuleInfo};
pub use biome_css_syntax::EmbeddingStyleApplicability;
pub use biome_js_type_info::ImportSymbol;
pub use biome_resolver::ResolvedPath;
pub use db::inputs::ModuleDb;
pub use diagnostics::ModuleDiagnostic;
pub use js_module_info::{
    BindingTypeData, JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo,
    JsModuleInfoDiagnostic, JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
    resolve_js_module,
};
pub use module_graph::{
    ModuleDependencies, ModuleGraph, ModuleInfo, ModuleInfoKind, SUPPORTED_EXTENSIONS,
    SerializedModuleInfo, resolve_css_module, resolve_html_module,
};
pub use path_info_cache::{PathInfoCache, prepopulate_directory_path_info};
