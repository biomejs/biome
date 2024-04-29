mod errors;
mod grit_binding;
mod grit_code_snippet;
mod grit_context;
mod grit_file;
mod grit_language;
mod grit_node;
mod grit_node_patterns;
mod grit_tree;
mod pattern;
mod pattern_compiler;
mod resolved_pattern;

use biome_grit_parser::parse_grit;
pub use errors::*;
pub use pattern::GritPattern;

/// Compiles a Grit pattern from the given source string.
pub fn compile_pattern(source: &str) -> Result<GritPattern, CompileError> {
    let parsed = parse_grit(source);
    GritPattern::from_node(parsed.tree())
}
