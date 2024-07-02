//! Extremely fast, lossless, and error tolerant GraphQL Parser.

use biome_graphql_factory::GraphqlSyntaxFactory;
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode};
pub use biome_parser::prelude::*;
use biome_parser::{tree_sink::LosslessTreeSink, AnyParse};
use biome_rowan::{AstNode, NodeCache};
use parser::{parse_root, GraphqlParser};

mod lexer;
mod parser;
mod token_source;

pub(crate) type GraphqlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, GraphqlLanguage, GraphqlSyntaxFactory>;

pub fn parse_graphql(source: &str) -> GraphqlParse {
    let mut cache = NodeCache::default();
    parse_graphql_with_cache(source, &mut cache)
}

/// Parses the provided string as Graphql program using the provided node cache.
pub fn parse_graphql_with_cache(source: &str, cache: &mut NodeCache) -> GraphqlParse {
    tracing::debug_span!("Parsing phase").in_scope(move || {
        let mut parser = GraphqlParser::new(source);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = GraphqlLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        GraphqlParse::new(green, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct GraphqlParse {
    root: GraphqlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl GraphqlParse {
    pub fn new(root: GraphqlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> GraphqlParse {
        GraphqlParse { root, diagnostics }
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
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_graphql;

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _graphql = parse_graphql(src);
    }
}
