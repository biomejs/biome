#![deny(clippy::use_self)]
#![expect(
    unused_lifetimes,
    reason = "Salsa interned structs can use the database lifetime only in generated field storage."
)]

pub mod css_module_info;
mod db;
mod diagnostics;
mod format_module_graph;
mod html_module_info;
mod import_path_map;
pub mod js_module_info;
mod module_graph;
mod path_info_cache;
mod traverse;
pub mod type_inference;

pub use biome_js_type_info::{
    ImportSymbol,
    resolved::{InferredLocalTypeId, InferredModuleKey},
};
pub use biome_resolver::ResolvedPath;
pub use css_module_info::{
    CssClassReference, CssClassStep, CssImport, CssImports, CssModuleInfo, CssPropertyDefinition,
    CssTraversalStep, ImportTreeDisplay, ImportTreeNode,
};
pub use db::queries::{
    BindingTypeInput, CallArgumentTypeInput, CallExpressionTypeInput, ExpressionTypeInput,
    InferredModuleTypes, JsModuleSccs, LocalTypeInput, NormalizeTypeInput, SymbolFromModuleInfo,
    build_import_tree_for_html, build_import_tree_for_js, css_classes_for_module,
    css_property_definitions, find_css_class_definition, find_js_exported_symbol,
    find_jsdoc_for_exported_symbol, find_member_type, find_value_member_type,
    function_returns_promise, infer_binding_type, infer_call_argument_type,
    infer_call_expression_type, infer_constructor_argument_type, infer_export_type,
    infer_expression_function_returns_promise, infer_expression_is_array_of_promises,
    infer_expression_is_promise, infer_expression_type, infer_local_type, infer_module_types,
    infer_module_types_bottom_up, is_array_of_promise_type, is_class_referenced_by_importers,
    is_promise_type, js_module_sccs, normalize_type, resolve_callable_type,
    transitive_importers_of, traverse_import_tree_for_classes,
    traverse_import_tree_for_html_classes,
};
pub use db::{ModuleDb, ModuleGraphGeneration, TypeDb, module_for_key};
pub use diagnostics::ModuleDiagnostic;
pub use html_module_info::{
    HtmlEmbeddedContent, HtmlImport, HtmlModuleInfo, SerializedHtmlModuleInfo,
};
pub use import_path_map::{ImportPathMap, ImportPathMapIterator};
pub use js_module_info::{
    JsExport, JsExportedSymbolLookup, JsImport, JsImportKind, JsImportPath, JsImportPhase,
    JsModuleInfo, JsModuleInfoDiagnostic, JsOwnExport, JsReexport, SerializedJsModuleInfo,
    TypeInferenceMode,
};
pub use module_graph::{
    ModuleDependencies, ModuleInfo, ModuleInfoKind, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
    resolve_css_module, resolve_html_module, resolve_js_module,
    resolve_js_module_with_inference_mode,
};
pub use path_info_cache::PathInfoCache;
