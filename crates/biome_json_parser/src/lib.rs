//! Extremely fast, lossless, and error tolerant JSON Parser.

use crate::parser::JsonParser;
use crate::syntax::parse_root;
use biome_json_factory::JsonSyntaxFactory;
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
pub use biome_parser::prelude::*;
use biome_parser::{tree_sink::LosslessTreeSink, AnyParse};
use biome_rowan::{AstNode, NodeCache};
pub use parser::JsonParserOptions;

mod lexer;
mod parser;
mod prelude;
mod syntax;
mod token_source;

pub(crate) type JsonLosslessTreeSink<'source> =
    LosslessTreeSink<'source, JsonLanguage, JsonSyntaxFactory>;

pub fn parse_json(source: &str, options: JsonParserOptions) -> JsonParse {
    let mut cache = NodeCache::default();
    parse_json_with_cache(source, &mut cache, options)
}

/// Parses the provided string as JSON program using the provided node cache.
pub fn parse_json_with_cache(
    source: &str,
    cache: &mut NodeCache,
    config: JsonParserOptions,
) -> JsonParse {
    tracing::debug_span!("parse").in_scope(move || {
        let mut parser = JsonParser::new(source, config);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = JsonLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        JsonParse::new(green, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct JsonParse {
    root: JsonSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl JsonParse {
    pub fn new(root: JsonSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> JsonParse {
        JsonParse { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_json_parser::parse_json;
    /// # use biome_json_syntax::JsonSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_json_syntax::JsonSyntaxKind;
    /// use biome_json_parser::JsonParserOptions;
    /// let parse = parse_json(r#"["a", 1]"#, JsonParserOptions::default());
    ///
    /// // Get the root value
    /// let root_value = parse.tree().value()?;
    ///
    /// assert_eq!(root_value.syntax().kind(), JsonSyntaxKind::JSON_ARRAY_VALUE);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> JsonSyntaxNode {
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
    pub fn tree(&self) -> JsonRoot {
        JsonRoot::unwrap_cast(self.syntax())
    }
}

impl From<JsonParse> for AnyParse {
    fn from(parse: JsonParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}
