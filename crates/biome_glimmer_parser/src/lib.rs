#![deny(clippy::use_self)]

mod lexer;
mod parser;
mod syntax;
mod token_source;

pub use parser::GlimmerParseOptions;

use crate::parser::{GlimmerLosslessTreeSink, GlimmerParser};
use crate::syntax::parse_root;
use biome_glimmer_syntax::{GlimmerRoot, GlimmerSyntaxNode};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::{AnyParse, NodeParse};
use biome_rowan::{AstNode, NodeCache};

/// Parses the provided string as a Glimmer template using the provided node cache.
pub fn parse_glimmer_with_cache(
    source: &str,
    cache: &mut NodeCache,
    options: GlimmerParseOptions,
) -> GlimmerParse {
    let mut parser = GlimmerParser::new(source, options);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = GlimmerLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    GlimmerParse::new(green, diagnostics)
}

/// Parses a Glimmer template with the provided options
pub fn parse_glimmer(source: &str, options: GlimmerParseOptions) -> GlimmerParse {
    let mut cache = NodeCache::default();
    parse_glimmer_with_cache(source, &mut cache, options)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct GlimmerParse {
    root: GlimmerSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GlimmerParse {
    pub fn new(root: GlimmerSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> GlimmerSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    ///
    /// It panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> GlimmerRoot {
        GlimmerRoot::unwrap_cast(self.syntax())
    }
}

impl From<GlimmerParse> for AnyParse {
    fn from(parse: GlimmerParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        NodeParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
        .into()
    }
}
