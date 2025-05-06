#![deny(clippy::use_self)]

mod format_module_graph;
mod js_module_info;
pub mod jsdoc_comment;
mod module_graph;
mod resolver_cache;

pub use biome_js_type_info::{ImportSymbol, ResolvedPath};

pub use js_module_info::{
    JsExport, JsImport, JsModuleInfo, JsOwnExport, JsReexport, ScopedResolver,
};
pub use jsdoc_comment::JsdocComment;
pub use module_graph::{ModuleGraph, SUPPORTED_EXTENSIONS};
