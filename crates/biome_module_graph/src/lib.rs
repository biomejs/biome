#![deny(clippy::use_self)]

mod css_module_info;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
mod js_module_info;
mod module_graph;

pub use biome_js_type_info::ImportSymbol;
pub use biome_resolver::ResolvedPath;

pub use css_module_info::{CssClass, CssImport, CssImports, CssModuleInfo};
pub use diagnostics::ModuleDiagnostic;
pub use html_module_info::{HtmlModuleInfo, SerializedHtmlModuleInfo};
pub use js_module_info::{
    JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo, JsModuleInfoDiagnostic,
    JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
};
pub use module_graph::{
    ModuleDependencies, ModuleGraph, ModuleInfo, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
};
