use biome_markdown_factory::MarkdownSyntaxFactory;
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode, MdDocument};
use biome_parser::{prelude::ParseDiagnostic, tree_sink::LosslessTreeSink};
use biome_rowan::{AstNode, NodeCache};
use parser::MarkdownParser;
use syntax::parse_document;

mod lexer;
mod parser;
mod syntax;
mod token_source;

pub(crate) type MarkdownLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MarkdownLanguage, MarkdownSyntaxFactory>;

pub fn parse_markdown(source: &str) -> MarkdownParse {
    let mut cache = NodeCache::default();
    parse_markdown_with_cache(source, &mut cache)
}

pub fn parse_markdown_with_cache(source: &str, cache: &mut NodeCache) -> MarkdownParse {
    tracing::debug_span!("Parsing phase").in_scope(move || {
        let mut parser = MarkdownParser::new(source);

        parse_document(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = MarkdownLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        MarkdownParse::new(green, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct MarkdownParse {
    root: MarkdownSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl MarkdownParse {
    pub fn new(root: MarkdownSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> MarkdownParse {
        MarkdownParse { root, diagnostics }
    }

    pub fn syntax(&self) -> MarkdownSyntaxNode {
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
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> MdDocument {
        MdDocument::unwrap_cast(self.syntax())
    }
}
