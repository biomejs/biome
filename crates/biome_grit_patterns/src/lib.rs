#![allow(dead_code)] // FIXME: Remove when more stuff is ready
mod diagnostics;
mod errors;
mod grit_analysis_ext;
mod grit_binding;
mod grit_code_snippet;
mod grit_context;
mod grit_file;
mod grit_js_parser;
mod grit_node;
mod grit_node_patterns;
mod grit_query;
mod grit_resolved_pattern;
mod grit_target_language;
mod grit_target_node;
mod grit_tree;
mod pattern_compiler;
mod source_location_ext;
mod util;
mod variables;

pub use errors::*;
pub use grit_query::GritQuery;
pub use grit_target_language::{GritTargetLanguage, JsTargetLanguage};
pub use grit_tree::GritTargetTree;

use biome_grit_parser::parse_grit;

/// Compiles a Grit pattern from the given source string.
pub fn compile_pattern(
    source: &str,
    language: GritTargetLanguage,
) -> Result<GritQuery, CompileError> {
    let parsed = parse_grit(source);
    GritQuery::from_node(parsed.tree(), language)
}
