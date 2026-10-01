#![deny(clippy::use_self)]

mod constants;
mod lexer;
mod parser;
mod token_source;

use biome_grit_factory::GritSyntaxFactory;
use biome_grit_syntax::{GritLanguage, GritRoot, GritSyntaxNode};
use biome_parser::tree_sink::{LosslessTreeSink, OffsetLosslessTreeSink};
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse, diagnostic::ParseDiagnostic};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset, TextSize};
use parser::{GritParser, parse_root};

pub(crate) type GritLosslessTreeSink<'source> =
    LosslessTreeSink<'source, GritLanguage, GritSyntaxFactory>;
pub(crate) type GritOffsetLosslessTreeSink<'source> =
    OffsetLosslessTreeSink<'source, GritLanguage, GritSyntaxFactory>;

pub fn parse_grit(source: &str) -> GritParse {
    let mut cache = NodeCache::default();
    parse_grit_with_cache(source, &mut cache)
}

/// Parses the provided string as a GritQL pattern using the provided node cache.
pub fn parse_grit_with_cache(source: &str, cache: &mut NodeCache) -> GritParse {
    let mut parser = GritParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = GritLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    GritParse::new(green, diagnostics)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct GritParse {
    root: GritSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GritParse {
    pub fn new(root: GritSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_grit_parser::parse_grit;
    /// # use biome_grit_syntax::{GritSyntaxKind, AnyGritLiteral, AnyGritPattern, GritRootExt};
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() {
    /// use biome_grit_syntax::GritSyntaxKind;
    /// let parse = parse_grit(r#"`console.log($message)`"#);
    ///
    /// // Get the pattern
    /// let pattern = parse.tree().pattern();
    /// match pattern {
    ///     Some(AnyGritPattern::AnyGritLiteral(AnyGritLiteral::GritCodeSnippet(snippet))) => {
    ///         assert_eq!(
    ///             snippet.source().unwrap().syntax().kind(),
    ///             GritSyntaxKind::GRIT_BACKTICK_SNIPPET_LITERAL
    ///         );
    ///     }
    ///     _ => panic!("Unexpected pattern"),
    /// }
    /// # }
    /// ```
    pub fn syntax(&self) -> GritSyntaxNode {
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
    pub fn tree(&self) -> GritRoot {
        GritRoot::unwrap_cast(self.syntax())
    }
}

impl From<GritParse> for AnyParse {
    fn from(parse: GritParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        NodeParse::new(root.as_send().unwrap(), diagnostics).into()
    }
}

/// Parses GritQL `source` with a `base_offset` for embedded content.
pub fn parse_grit_with_offset(source: &str, base_offset: TextSize) -> GritOffsetParse {
    parse_grit_with_offset_and_cache(source, base_offset, &mut NodeCache::default())
}

/// Parses GritQL `source` with a `base_offset` and `cache` for embedded content.
pub fn parse_grit_with_offset_and_cache(
    source: &str,
    base_offset: TextSize,
    cache: &mut NodeCache,
) -> GritOffsetParse {
    let mut parser = GritParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = GritOffsetLosslessTreeSink::with_cache(source, &trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (root, diagnostics) = tree_sink.finish();

    GritOffsetParse::new(root, diagnostics)
}

/// A utility struct for managing the result of an offset-aware GritQL parser job.
#[derive(Clone, Debug)]
pub struct GritOffsetParse {
    root: SyntaxNodeWithOffset<GritLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GritOffsetParse {
    pub fn new(
        root: SyntaxNodeWithOffset<GritLanguage>,
        diagnostics: Vec<ParseDiagnostic>,
    ) -> Self {
        Self { root, diagnostics }
    }

    /// Returns the offset-aware syntax node represented by this parse result.
    pub fn syntax(&self) -> SyntaxNodeWithOffset<GritLanguage> {
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

    /// Converts this parse into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> GritRoot {
        GritRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Returns the base offset applied to this parse result.
    pub fn base_offset(&self) -> TextSize {
        self.root.base_offset()
    }

    /// Converts back to the underlying parse result, discarding offset information.
    pub fn into_inner(self) -> GritParse {
        GritParse::new(self.root.into_inner(), self.diagnostics)
    }
}

impl From<GritOffsetParse> for AnyParse {
    fn from(parse: GritOffsetParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        EmbeddedNodeParse::new(root.as_embedded_send(), diagnostics).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_grit_with_offset;
    use biome_rowan::TextSize;

    #[test]
    fn offset_parse_ranges_start_at_base_offset() {
        let source = "`console.log($message)`";
        let base_offset = TextSize::from(42);
        let parse = parse_grit_with_offset(source, base_offset);

        assert!(!parse.has_errors());
        assert_eq!(parse.base_offset(), base_offset);
        assert_eq!(parse.syntax().text_range_with_trivia().start(), base_offset);
        assert_eq!(
            parse.syntax().text_range_with_trivia().end(),
            base_offset + TextSize::from(source.len() as u32)
        );
    }
}
