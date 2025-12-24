//! Extremely fast, lossless, and error tolerant GraphQL Parser.

#![deny(clippy::use_self)]

use biome_graphql_factory::GraphqlSyntaxFactory;
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode};
pub use biome_parser::prelude::*;
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset};
use parser::{GraphqlParser, parse_root};

mod lexer;
mod parser;
mod token_source;

pub(crate) type GraphqlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, GraphqlLanguage, GraphqlSyntaxFactory>;

pub(crate) type GraphqlOffsetLosslessTreeSink<'source> =
    OffsetLosslessTreeSink<'source, GraphqlLanguage, GraphqlSyntaxFactory>;

pub fn parse_graphql(source: &str) -> GraphqlParse {
    let mut cache = NodeCache::default();
    parse_graphql_with_cache(source, &mut cache)
}

/// Parses the provided string as Graphql program using the provided node cache.
pub fn parse_graphql_with_cache(source: &str, cache: &mut NodeCache) -> GraphqlParse {
    let mut parser = GraphqlParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = GraphqlLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    GraphqlParse::new(green, diagnostics)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct GraphqlParse {
    root: GraphqlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GraphqlParse {
    pub fn new(root: GraphqlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_graphql_parser::parse_graphql;
    /// # use biome_graphql_syntax::GraphqlSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_graphql_syntax::GraphqlSyntaxKind;
    /// let parse = parse_graphql(r#""#);
    ///
    /// let root_value = parse.tree().definitions();
    ///
    /// assert_eq!(root_value.syntax().kind(), GraphqlSyntaxKind::GRAPHQL_DEFINITION_LIST);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> GraphqlSyntaxNode {
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
    pub fn tree(&self) -> GraphqlRoot {
        GraphqlRoot::unwrap_cast(self.syntax())
    }
}

impl From<GraphqlParse> for AnyParse {
    fn from(parse: GraphqlParse) -> Self {
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

/// Parses GraphQL code with an offset for embedded content.
///
/// This function is designed for parsing embedded GraphQL content (e.g., in JavaScript `gql` tag)
/// where the source positions need to be adjusted relative to the parent document.
///
/// # Arguments
/// * `source` - The GraphQL source code to parse
/// * `base_offset` - The offset to apply to all source positions
///
/// # Examples
/// ```
/// use biome_graphql_parser::parse_graphql_with_offset;
/// use biome_rowan::TextSize;
///
/// // Parsing embedded GraphQL starting at position 50 in a JavaScript document
/// let graphql_code = "query {}";
/// let offset = TextSize::from(50);
/// let parse = parse_graphql_with_offset(graphql_code, offset);
///
/// // All text ranges in the resulting AST will be offset by 50
/// assert_eq!(parse.base_offset(), offset);
/// ```
pub fn parse_graphql_with_offset(
    source: &str,
    base_offset: biome_rowan::TextSize,
) -> GraphqlOffsetParse {
    let mut cache = NodeCache::default();
    parse_graphql_with_offset_and_cache(source, base_offset, &mut cache)
}

/// Parses GraphQL code with an offset and cache for embedded content.
///
/// This is the cache-enabled version of [`parse_graphql_with_offset`] for improved performance
/// when parsing multiple embedded GraphQL blocks.
pub fn parse_graphql_with_offset_and_cache(
    source: &str,
    base_offset: biome_rowan::TextSize,
    cache: &mut NodeCache,
) -> GraphqlOffsetParse {
    let mut parser = GraphqlParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink =
        GraphqlOffsetLosslessTreeSink::with_cache(source, &trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (offset_node, parse_diagnostics) = tree_sink.finish();

    GraphqlOffsetParse::new(offset_node, parse_diagnostics)
}

/// A utility struct for managing the result of an offset-aware GraphQL parser job
#[derive(Clone, Debug)]
pub struct GraphqlOffsetParse {
    root: SyntaxNodeWithOffset<GraphqlLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GraphqlOffsetParse {
    pub fn new(
        root: SyntaxNodeWithOffset<GraphqlLanguage>,
        diagnostics: Vec<ParseDiagnostic>,
    ) -> Self {
        Self { root, diagnostics }
    }

    /// The offset-aware syntax node represented by this Parse result
    pub fn syntax(&self) -> SyntaxNodeWithOffset<GraphqlLanguage> {
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
    pub fn tree(&self) -> GraphqlRoot {
        GraphqlRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Get the base offset applied to this parse result
    pub fn base_offset(&self) -> biome_rowan::TextSize {
        self.root.base_offset()
    }

    /// Convert back to the underlying parse result, discarding offset information
    pub fn into_inner(self) -> GraphqlParse {
        GraphqlParse::new(self.root.into_inner(), self.diagnostics)
    }
}

impl From<GraphqlOffsetParse> for AnyParse {
    fn from(parse: GraphqlOffsetParse) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::{parse_graphql, parse_graphql_with_offset};
    use biome_graphql_syntax::TextSize;

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _graphql = parse_graphql(src);
    }

    #[test]
    fn test_graphql_offset_parsing_basic() {
        let graphql_code = "query {}";
        let base_offset = TextSize::from(75);

        let parse = parse_graphql_with_offset(graphql_code, base_offset);

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
        let expected_end = base_offset + TextSize::from(graphql_code.len() as u32);
        assert_eq!(root_range.end(), expected_end);
    }
}
