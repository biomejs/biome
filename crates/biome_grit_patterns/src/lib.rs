mod diagnostics;
mod errors;
mod grit_analysis_ext;
mod grit_binding;
mod grit_built_in_functions;
mod grit_code_snippet;
mod grit_context;
mod grit_definitions;
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
pub use grit_context::GritTargetFile;
pub use grit_query::{CreateFile, GritQuery, GritQueryResult, Message, OutputFile};
pub use grit_target_language::{GritTargetLanguage, JsTargetLanguage};

use biome_grit_parser::parse_grit;
use std::path::Path;

/// Compiles a Grit pattern from the given source string.
pub fn compile_pattern(
    source: &str,
    path: Option<&Path>,
    language: GritTargetLanguage,
) -> Result<GritQuery, CompileError> {
    let parsed = parse_grit(source);
    if parsed.has_errors() {
        return Err(CompileError::ParsePatternError(
            // TODO: We may want to preserve other diagnostics too.
            parsed.into_diagnostics().remove(0),
        ));
    }

    GritQuery::from_node(parsed.tree(), path, language)
}
