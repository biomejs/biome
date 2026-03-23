#![deny(clippy::use_self)]

mod lexer;
mod parser;
mod syntax;
mod token_source;

use crate::parser::{TailwindLosslessTreeSink, TailwindOffsetLosslessTreeSink, TailwindParser};
use crate::syntax::parse_root;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset, TextSize};
use biome_tailwind_syntax::{TailwindLanguage, TailwindSyntaxNode, TwRoot};

/// Parses the provided string as Tailwind using the provided node cache.
pub fn parse_tailwind_with_cache(source: &str, cache: &mut NodeCache) -> TailwindParse {
    let mut parser = TailwindParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = TailwindLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    TailwindParse::new(green, diagnostics)
}
pub fn parse_tailwind(source: &str) -> TailwindParse {
    let mut cache = NodeCache::default();
    parse_tailwind_with_cache(source, &mut cache)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct TailwindParse {
    root: TailwindSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl TailwindParse {
    pub fn new(root: TailwindSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_tailwind_parser::parse_tailwind;
    /// # use biome_tailwind_syntax::TailwindSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_tailwind_syntax::TailwindSyntaxKind;
    /// let parse = parse_tailwind(r#"bg-red-500"#);
    ///
    /// // Get the root value
    /// let root_value = parse.tree();
    ///
    /// // assert_eq!(root_value.syntax().kind(), TailwindSyntaxKind::TW_ROOT);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> TailwindSyntaxNode {
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
    pub fn tree(&self) -> TwRoot {
        TwRoot::unwrap_cast(self.syntax())
    }
}

impl From<TailwindParse> for AnyParse {
    fn from(parse: TailwindParse) -> Self {
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

/// A utility struct for managing the result of an offset-aware Tailwind parser job
#[derive(Clone, Debug)]
pub struct TailwindOffsetParse {
    root: SyntaxNodeWithOffset<TailwindLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl TailwindOffsetParse {
    pub fn new(
        root: SyntaxNodeWithOffset<TailwindLanguage>,
        diagnostics: Vec<ParseDiagnostic>,
    ) -> Self {
        Self { root, diagnostics }
    }

    /// The offset-aware syntax node represented by this Parse result
    pub fn syntax(&self) -> SyntaxNodeWithOffset<TailwindLanguage> {
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
    pub fn tree(&self) -> TwRoot {
        TwRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Get the base offset applied to this parse result
    pub fn base_offset(&self) -> TextSize {
        self.root.base_offset()
    }

    /// Convert back to the underlying parse result, discarding offset information
    pub fn into_inner(self) -> TailwindParse {
        TailwindParse::new(self.root.into_inner(), self.diagnostics)
    }
}

impl From<TailwindOffsetParse> for AnyParse {
    fn from(parse: TailwindOffsetParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        EmbeddedNodeParse::new(
            // SAFETY: the parser should always return a root node
            root.as_embedded_send(),
            diagnostics,
        )
        .into()
    }
}

/// Parses Tailwind code with an offset for embedded content.
///
/// This function is designed for parsing embedded Tailwind class strings
/// where the source positions need to be adjusted relative to the parent document.
pub fn parse_tailwind_with_offset(source: &str, base_offset: TextSize) -> TailwindOffsetParse {
    let mut cache = NodeCache::default();
    parse_tailwind_with_offset_and_cache(source, base_offset, &mut cache)
}

/// Parses Tailwind code with an offset and cache for embedded content.
pub fn parse_tailwind_with_offset_and_cache(
    source: &str,
    base_offset: TextSize,
    cache: &mut NodeCache,
) -> TailwindOffsetParse {
    let mut parser = TailwindParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink =
        TailwindOffsetLosslessTreeSink::with_cache(source, &trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (offset_node, parse_diagnostics) = tree_sink.finish();

    TailwindOffsetParse::new(offset_node, parse_diagnostics)
}
