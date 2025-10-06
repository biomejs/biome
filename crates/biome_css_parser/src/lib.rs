//! Extremely fast, lossless, and error-tolerant CSS Parser.

#![deny(clippy::use_self)]

use crate::parser::CssParser;
use crate::syntax::parse_root;
use biome_css_factory::CssSyntaxFactory;
use biome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
pub use biome_parser::prelude::*;
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset};
pub use parser::CssParserOptions;

mod lexer;
mod parser;
mod prelude;
mod state;
mod syntax;
mod token_source;

pub(crate) type CssLosslessTreeSink<'source> =
    LosslessTreeSink<'source, CssLanguage, CssSyntaxFactory>;

pub(crate) type CssOffsetLosslessTreeSink<'source> =
    biome_parser::tree_sink::OffsetLosslessTreeSink<'source, CssLanguage, CssSyntaxFactory>;

pub fn parse_css(source: &str, options: CssParserOptions) -> CssParse {
    let mut cache = NodeCache::default();
    parse_css_with_cache(source, &mut cache, options)
}

/// Parses the provided string as CSS program using the provided node cache.
pub fn parse_css_with_cache(
    source: &str,
    cache: &mut NodeCache,
    options: CssParserOptions,
) -> CssParse {
    let mut parser = CssParser::new(source, options);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = CssLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    CssParse::new(green, diagnostics)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct CssParse {
    root: CssSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl CssParse {
    pub fn new(root: CssSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_css_parser::parse_css;
    /// # use biome_css_syntax::CssSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_css_syntax::CssSyntaxKind;
    /// use biome_css_parser::CssParserOptions;
    /// let parse = parse_css(r#""#, CssParserOptions::default());
    ///
    /// let root_value = parse.tree().rules();
    ///
    /// assert_eq!(root_value.syntax().kind(), CssSyntaxKind::CSS_RULE_LIST);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> CssSyntaxNode {
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
    pub fn tree(&self) -> CssRoot {
        CssRoot::unwrap_cast(self.syntax())
    }
}

impl From<CssParse> for AnyParse {
    fn from(parse: CssParse) -> Self {
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

/// A utility struct for managing the result of an offset-aware CSS parser job
#[derive(Clone, Debug)]
pub struct CssOffsetParse {
    root: SyntaxNodeWithOffset<CssLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl CssOffsetParse {
    pub fn new(root: SyntaxNodeWithOffset<CssLanguage>, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The offset-aware syntax node represented by this Parse result
    pub fn syntax(&self) -> SyntaxNodeWithOffset<CssLanguage> {
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

    /// Convert this parse into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> CssRoot {
        CssRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Get the base offset applied to this parse result
    pub fn base_offset(&self) -> biome_rowan::TextSize {
        self.root.base_offset()
    }

    /// Convert back to the underlying parse result, discarding offset information
    pub fn into_inner(self) -> CssParse {
        CssParse::new(self.root.into_inner(), self.diagnostics)
    }
}

impl From<CssOffsetParse> for AnyParse {
    fn from(parse: CssOffsetParse) -> Self {
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

/// Parses CSS code with an offset for embedded content.
///
/// This function is designed for parsing embedded CSS content (e.g., in HTML `<style>` tags)
/// where the source positions need to be adjusted relative to the parent document.
///
/// # Arguments
/// * `source` - The CSS source code to parse
/// * `base_offset` - The offset to apply to all source positions
/// * `options` - Parser options
///
/// # Examples
/// ```
/// use biome_css_parser::{CssParserOptions, parse_css_with_offset};
/// use biome_rowan::TextSize;
///
/// // Parsing embedded CSS starting at position 50 in an HTML document
/// let css_code = "body { color: red; }";
/// let offset = TextSize::from(50);
/// let parse = parse_css_with_offset(css_code, offset, CssParserOptions::default());
///
/// // All text ranges in the resulting AST will be offset by 50
/// assert_eq!(parse.base_offset(), offset);
/// ```
pub fn parse_css_with_offset(
    source: &str,
    base_offset: biome_rowan::TextSize,
    options: CssParserOptions,
) -> CssOffsetParse {
    let mut cache = NodeCache::default();
    parse_css_with_offset_and_cache(source, base_offset, &mut cache, options)
}

/// Parses CSS code with an offset and cache for embedded content.
///
/// This is the cache-enabled version of [`parse_css_with_offset`] for improved performance
/// when parsing multiple embedded CSS blocks.
pub fn parse_css_with_offset_and_cache(
    source: &str,
    base_offset: biome_rowan::TextSize,
    cache: &mut NodeCache,
    options: CssParserOptions,
) -> CssOffsetParse {
    let mut parser = CssParser::new(source, options);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = CssOffsetLosslessTreeSink::with_cache(source, &trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (offset_node, parse_diagnostics) = tree_sink.finish();

    CssOffsetParse::new(offset_node, parse_diagnostics)
}

#[cfg(test)]
mod tests {
    use crate::{CssParserOptions, parse_css};
    use crate::{parse_css_with_cache, parse_css_with_offset};
    use biome_rowan::TextSize;

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _css = parse_css(src, CssParserOptions::default());
    }

    #[test]
    fn test_css_offset_parsing_basic() {
        let css_code = "body { color: red; }";
        let base_offset = TextSize::from(75);

        let parse = parse_css_with_offset(css_code, base_offset, CssParserOptions::default());

        // Verify no parsing errors
        assert!(!parse.has_errors(), "Parse should not have errors");

        // Verify the base offset is correctly set
        assert_eq!(parse.base_offset(), base_offset);

        // Verify the syntax tree text ranges are offset
        let syntax = parse.syntax();
        let root_range = syntax.text_range_with_trivia();

        // The root should start at the base offset
        assert_eq!(root_range.start(), base_offset);

        // The end should be base_offset + length of the text
        let expected_end = base_offset + TextSize::from(css_code.len() as u32);
        assert_eq!(root_range.end(), expected_end);
    }

    #[test]
    fn test_css_offset_parsing_vs_regular_parsing() {
        let css_code = ".container { width: 100%; margin: 0 auto; }";
        let base_offset = TextSize::from(25);

        // Parse with offset
        let offset_parse =
            parse_css_with_offset(css_code, base_offset, CssParserOptions::default());

        // Parse normally
        let normal_parse = parse_css_with_cache(
            css_code,
            &mut biome_rowan::NodeCache::default(),
            CssParserOptions::default(),
        );

        // Both should have same number of errors (hopefully none)
        assert_eq!(offset_parse.has_errors(), normal_parse.has_errors());

        // The offset parse should have all ranges shifted by base_offset
        let offset_range = offset_parse.syntax().text_range_with_trivia();
        let normal_range = normal_parse.syntax().text_range_with_trivia();

        assert_eq!(offset_range.start(), normal_range.start() + base_offset);
        assert_eq!(offset_range.end(), normal_range.end() + base_offset);

        // The text content should be identical
        assert_eq!(
            offset_parse.syntax().inner().text_with_trivia().to_string(),
            normal_parse.syntax().text_with_trivia().to_string()
        );
    }
}
