mod diagnostics;
mod errors;
mod grit_analysis_ext;
mod grit_binding;
mod grit_built_in_functions;
mod grit_code_snippet;
mod grit_context;
mod grit_css_parser;
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

use biome_grit_syntax::AnyGritLanguageDeclaration;
pub use errors::*;
pub use grit_binding::GritBinding;
pub use grit_built_in_functions::BuiltInFunction;
pub use grit_context::{GritExecContext, GritQueryContext, GritTargetFile};
pub use grit_pattern_matcher::pattern::{Pattern as GritPattern, State as GritQueryState};
pub use grit_query::{
    CreateFile, GritQuery, GritQueryEffect, GritQueryResult, Message, OutputFile,
};
pub use grit_resolved_pattern::GritResolvedPattern;
pub use grit_target_language::{GritTargetLanguage, JsTargetLanguage};
pub use grit_target_node::{GritTargetLanguageNode, GritTargetNode, GritTargetSyntaxKind};

use biome_grit_parser::parse_grit;
use camino::Utf8Path;

/// Compiles a Grit pattern from the given source string with default options.
pub fn compile_pattern(source: &str) -> Result<GritQuery, CompileError> {
    compile_pattern_with_options(source, CompilePatternOptions::default())
}

/// Compiles a Grit pattern from the given source string with the given options.
pub fn compile_pattern_with_options(
    source: &str,
    options: CompilePatternOptions,
) -> Result<GritQuery, CompileError> {
    let parsed = parse_grit(source);
    if parsed.has_errors() {
        return Err(CompileError::ParsePatternError(
            // TODO: We may want to preserve other diagnostics too.
            parsed.into_diagnostics().remove(0),
        ));
    }

    let language = parsed
        .tree()
        .language()
        .as_ref()
        .and_then(AnyGritLanguageDeclaration::as_grit_language_declaration)
        .and_then(GritTargetLanguage::from_declaration)
        .unwrap_or(options.default_language);

    GritQuery::from_node(
        parsed.tree(),
        options.path,
        language,
        options.extra_built_ins,
    )
}

#[derive(Default)]
pub struct CompilePatternOptions<'a> {
    default_language: GritTargetLanguage,
    extra_built_ins: Vec<BuiltInFunction>,
    path: Option<&'a Utf8Path>,
}

impl<'a> CompilePatternOptions<'a> {
    pub fn with_default_language(mut self, default_language: GritTargetLanguage) -> Self {
        self.default_language = default_language;
        self
    }

    pub fn with_extra_built_ins(mut self, built_ins: Vec<BuiltInFunction>) -> Self {
        self.extra_built_ins = built_ins;
        self
    }

    pub fn with_path(mut self, path: &'a Utf8Path) -> Self {
        self.path = Some(path);
        self
    }
}
