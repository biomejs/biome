#![deny(clippy::use_self)]

use biome_markdown_factory::MarkdownSyntaxFactory;
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode, MdDocument};
use biome_parser::{prelude::ParseDiagnostic, tree_sink::LosslessTreeSink};
use biome_rowan::{AstNode, NodeCache};
use parser::MarkdownParser;
use syntax::parse_document;

mod lexer;
mod link_reference;
mod parser;
mod syntax;
mod token_source;

// Test utilities for CommonMark spec compliance testing.
// Only compiled when the `test_utils` feature is enabled.
#[cfg(feature = "test_utils")]
mod to_html;

pub use parser::MarkdownParseOptions;

#[cfg(feature = "test_utils")]
pub use to_html::document_to_html;

pub(crate) type MarkdownLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MarkdownLanguage, MarkdownSyntaxFactory>;

/// Parse markdown source code with default options.
pub fn parse_markdown(source: &str) -> MarkdownParse {
    let mut cache = NodeCache::default();
    parse_markdown_with_cache(source, &mut cache, MarkdownParseOptions::default())
}

/// Parse markdown source code with custom options and a node cache.
pub fn parse_markdown_with_cache(
    source: &str,
    cache: &mut NodeCache,
    options: MarkdownParseOptions,
) -> MarkdownParse {
    let link_definitions =
        link_reference::collect_link_reference_definitions(source, options.clone());
    let mut parser = MarkdownParser::new(source, options);
    parser.set_link_reference_definitions(link_definitions);

    parse_document(&mut parser);

    let (events, diagnostics, trivia, list_tightness, list_item_indents, quote_indents) =
        parser.finish();

    let mut tree_sink = MarkdownLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    MarkdownParse::new(
        green,
        diagnostics,
        list_tightness,
        list_item_indents,
        quote_indents,
    )
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct MarkdownParse {
    root: MarkdownSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
    list_tightness: Vec<parser::ListTightness>,
    list_item_indents: Vec<parser::ListItemIndent>,
    quote_indents: Vec<parser::QuoteIndent>,
}

impl MarkdownParse {
    pub fn new(
        root: MarkdownSyntaxNode,
        diagnostics: Vec<ParseDiagnostic>,
        list_tightness: Vec<parser::ListTightness>,
        list_item_indents: Vec<parser::ListItemIndent>,
        quote_indents: Vec<parser::QuoteIndent>,
    ) -> Self {
        Self {
            root,
            diagnostics,
            list_tightness,
            list_item_indents,
            quote_indents,
        }
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

    /// Returns the recorded tight/loose information for list nodes.
    pub fn list_tightness(&self) -> &[parser::ListTightness] {
        &self.list_tightness
    }

    pub fn list_item_indents(&self) -> &[parser::ListItemIndent] {
        &self.list_item_indents
    }

    pub fn quote_indents(&self) -> &[parser::QuoteIndent] {
        &self.quote_indents
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
