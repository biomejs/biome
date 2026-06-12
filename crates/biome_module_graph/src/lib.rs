#![deny(clippy::use_self)]

pub mod css_module_info;
mod db;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
pub mod js_module_info;
mod module_graph;
mod path_info_cache;

pub use biome_css_syntax::EmbeddingStyleApplicability;
pub use biome_js_type_info::ImportSymbol;
pub use biome_resolver::ResolvedPath;
pub use css_module_info::{
    CssClassReference, CssClassStep, CssImport, CssImports, CssModuleInfo, CssTraversalStep,
    ImportTreeDisplay, ImportTreeNode,
};
pub use db::ModuleDb;
pub use db::project_database::{ProjectDatabase, ProjectDatabaseHandle};
pub use db::queries::*;
pub use diagnostics::ModuleDiagnostic;
pub use html_module_info::{HtmlEmbeddedContent, HtmlModuleInfo, SerializedHtmlModuleInfo};
pub use js_module_info::{
    BindingTypeData, JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo,
    JsModuleInfoDiagnostic, JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
};
pub use module_graph::{
    ModuleDependencies, ModuleInfo, ModuleInfoKind, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
    resolve_css_module, resolve_html_module, resolve_js_module,
};
pub use path_info_cache::PathInfoCache;
