#![deny(clippy::use_self)]

mod css_module_info;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
mod js_module_info;
mod module_graph;

pub use crate::css_module_info::{
    CssClassReference, CssClassStep, CssImport, CssImports, CssModuleInfo, CssTraversalStep,
    ImportTreeDisplay, ImportTreeNode,
};
pub use crate::html_module_info::{HtmlEmbeddedContent, HtmlModuleInfo, SerializedHtmlModuleInfo};
pub use biome_css_syntax::EmbeddingStyleApplicability;
pub use biome_js_type_info::ImportSymbol;
pub use biome_resolver::ResolvedPath;
pub use diagnostics::ModuleDiagnostic;
pub use js_module_info::{
    BindingTypeData, JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo,
    JsModuleInfoDiagnostic, JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
};
pub use module_graph::{
    ModuleDependencies, ModuleGraph, ModuleInfo, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
};
