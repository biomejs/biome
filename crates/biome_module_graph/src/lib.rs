#![deny(clippy::use_self)]

pub mod css_module_info;
mod db;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
pub mod js_module_info;
mod module_graph;
mod path_info_cache;

pub use biome_js_type_info::{
    ImportSymbol,
    resolved::{InferredLocalTypeId, InferredModuleKey},
};
pub use biome_resolver::ResolvedPath;
pub use css_module_info::{
    CssClassReference, CssClassStep, CssImport, CssImports, CssModuleInfo, CssTraversalStep,
    ImportTreeDisplay, ImportTreeNode,
};
pub use db::queries::{
    CallArgumentTypeInput, CallExpressionTypeInput, InferredModuleTypes, NormalizeTypeInput,
    ResolvedCallArgument, SymbolFromModuleInfo, build_import_tree_for_html,
    build_import_tree_for_js, css_classes_for_module, find_css_class_definition,
    find_js_exported_symbol, find_jsdoc_for_exported_symbol, infer_call_argument_type,
    infer_call_expression_type, infer_constructor_argument_type, infer_module_types,
    infer_module_types_bottom_up, is_class_referenced_by_importers, normalize_type,
    transitive_importers_of, traverse_import_tree_for_classes,
    traverse_import_tree_for_html_classes,
};
pub use db::{ModuleDb, ModuleGraphGeneration, TypeDb};
pub use diagnostics::ModuleDiagnostic;
pub use html_module_info::{HtmlEmbeddedContent, HtmlModuleInfo, SerializedHtmlModuleInfo};
pub use js_module_info::{
    JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo, JsModuleInfoDiagnostic,
    JsOwnExport, JsReexport, SerializedJsModuleInfo, TypeInferenceMode,
};
pub use module_graph::{
    ModuleDependencies, ModuleInfo, ModuleInfoKind, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
    resolve_css_module, resolve_html_module, resolve_js_module,
    resolve_js_module_with_inference_mode,
};
pub use path_info_cache::PathInfoCache;
