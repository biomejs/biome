#![deny(clippy::use_self)]

use biome_markdown_factory::MarkdownSyntaxFactory;
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode, MdRoot};
use biome_parser::tree_sink::{LosslessTreeSink, OffsetLosslessTreeSink};
use biome_parser::{
    AnyParse, EmbeddedNodeParse, NodeParse, SyntaxFeature, prelude::ParseDiagnostic,
};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset, TextSize};
use parser::MarkdownParser;
use syntax::parse_document;

mod inline_phase;
mod lexer;
mod parser;
mod syntax;
mod token_source;

// Test utilities for CommonMark spec compliance testing.
// Only compiled when the `test_utils` feature is enabled.
#[cfg(feature = "test_utils")]
mod to_html;

pub use parser::MarkdownParserOptions;

#[cfg(feature = "test_utils")]
pub use to_html::document_to_html;

pub(crate) type MarkdownLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MarkdownLanguage, MarkdownSyntaxFactory>;
pub(crate) type MarkdownOffsetLosslessTreeSink<'source> =
    OffsetLosslessTreeSink<'source, MarkdownLanguage, MarkdownSyntaxFactory>;

fn parse_common(source: &str, options: &MarkdownParserOptions) -> parser::MarkdownParserOutput {
    let mut parser = MarkdownParser::new(source, options.clone());

    parse_document(&mut parser);

    let mut output = parser.finish();
    if !inline_phase::parse_deferred_inlines(source, options, &mut output) {
        output.deferred_inlines.clear();
    }
    debug_assert!(output.deferred_inlines.is_empty());
    output
}

/// Parse markdown source code with the given options.
pub fn parse_markdown(source: &str, options: MarkdownParserOptions) -> MarkdownParse {
    let mut cache = NodeCache::default();
    parse_markdown_with_cache(source, &mut cache, options)
}

/// Parse markdown source code with custom options and a node cache.
pub fn parse_markdown_with_cache(
    source: &str,
    cache: &mut NodeCache,
    options: MarkdownParserOptions,
) -> MarkdownParse {
    let output = parse_common(source, &options);

    let mut tree_sink = MarkdownLosslessTreeSink::with_cache(source, &output.trivia, cache);
    biome_parser::event::process(&mut tree_sink, output.events, output.diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    MarkdownParse::new(
        green,
        diagnostics,
        output.list_tightness,
        output.list_item_indents,
        output.quote_indents,
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
    pub fn tree(&self) -> MdRoot {
        MdRoot::unwrap_cast(self.syntax())
    }
}

impl From<MarkdownParse> for AnyParse {
    fn from(parse: MarkdownParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        NodeParse::new(root.as_send().unwrap(), diagnostics).into()
    }
}

/// Parses Markdown `source` with a `base_offset` for embedded content.
pub fn parse_markdown_with_offset(
    source: &str,
    base_offset: TextSize,
    options: MarkdownParserOptions,
) -> MarkdownOffsetParse {
    parse_markdown_with_offset_and_cache(source, base_offset, &mut NodeCache::default(), options)
}

/// Parses Markdown `source` with a `base_offset` and `cache` for embedded content.
pub fn parse_markdown_with_offset_and_cache(
    source: &str,
    base_offset: TextSize,
    cache: &mut NodeCache,
    options: MarkdownParserOptions,
) -> MarkdownOffsetParse {
    let output = parse_common(source, &options);

    let mut tree_sink =
        MarkdownOffsetLosslessTreeSink::with_cache(source, &output.trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, output.events, output.diagnostics);
    let (root, diagnostics) = tree_sink.finish();

    MarkdownOffsetParse::new(
        root,
        diagnostics,
        output.list_tightness,
        output.list_item_indents,
        output.quote_indents,
    )
}

/// A utility struct for managing the result of an offset-aware Markdown parser job.
#[derive(Clone, Debug)]
pub struct MarkdownOffsetParse {
    root: SyntaxNodeWithOffset<MarkdownLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
    list_tightness: Vec<parser::ListTightness>,
    list_item_indents: Vec<parser::ListItemIndent>,
    quote_indents: Vec<parser::QuoteIndent>,
}

impl MarkdownOffsetParse {
    pub fn new(
        root: SyntaxNodeWithOffset<MarkdownLanguage>,
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

    /// Returns the offset-aware syntax node represented by this parse result.
    pub fn syntax(&self) -> SyntaxNodeWithOffset<MarkdownLanguage> {
        self.root.clone()
    }

    /// Returns the diagnostics which occurred when parsing.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Retrieves the diagnostics which occurred when parsing.
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns `true` if the parser encountered errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Returns the recorded tight/loose information for list nodes.
    pub fn list_tightness(&self) -> &[parser::ListTightness] {
        &self.list_tightness
    }

    /// Returns the recorded indentation information for list items.
    pub fn list_item_indents(&self) -> &[parser::ListItemIndent] {
        &self.list_item_indents
    }

    /// Returns the recorded indentation information for block quotes.
    pub fn quote_indents(&self) -> &[parser::QuoteIndent] {
        &self.quote_indents
    }

    /// Converts this parse into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> MdRoot {
        MdRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Returns the base offset applied to this parse result.
    pub fn base_offset(&self) -> TextSize {
        self.root.base_offset()
    }

    /// Converts back to the underlying parse result, discarding offset information.
    pub fn into_inner(self) -> MarkdownParse {
        MarkdownParse::new(
            self.root.into_inner(),
            self.diagnostics,
            self.list_tightness,
            self.list_item_indents,
            self.quote_indents,
        )
    }
}

impl From<MarkdownOffsetParse> for AnyParse {
    fn from(parse: MarkdownOffsetParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        EmbeddedNodeParse::new(root.as_embedded_send(), diagnostics).into()
    }
}

pub(crate) enum MarkdownSyntaxFeatures {
    /// GitHub Flavored Markdown extensions.
    Gfm,
}

impl SyntaxFeature for MarkdownSyntaxFeatures {
    type Parser<'source> = MarkdownParser<'source>;

    fn is_supported(&self, p: &Self::Parser<'_>) -> bool {
        match self {
            Self::Gfm => p.options().gfm,
        }
    }
}
