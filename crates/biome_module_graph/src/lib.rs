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
pub use db::project_database::ProjectDatabase;
pub use db::queries::{
    collect_available_classes_for_js_file, transitive_importers_of,
    traverse_import_tree_for_classes, traverse_import_tree_for_html_classes,
};
pub use diagnostics::ModuleDiagnostic;
pub use js_module_info::{
    BindingTypeData, JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo,
    JsModuleInfoDiagnostic, JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
};
pub use module_graph::{
    ModuleDependencies, ModuleInfo, ModuleInfoKind, SUPPORTED_EXTENSIONS, SerializedModuleInfo,
    build_import_tree, build_import_tree_for_html, find_css_class_definition, find_exported_symbol,
    find_jsdoc_for_exported_symbol, is_class_referenced_by_importers, resolve_css_module,
    resolve_html_module, resolve_js_module,
};
pub use path_info_cache::PathInfoCache;
