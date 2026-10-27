//! Lossless, error tolerant TOML 1.1 parser.
//!
//! [`parse_toml`] reports every violation of the [TOML 1.1.0 specification](https://toml.io/en/v1.1.0):
//! syntax errors, invalid tokens, and keys or tables that are defined more than once.

#![deny(clippy::use_self)]

use crate::parser::TomlParser;
use crate::syntax::parse_root;
use biome_parser::AnyParse;
use biome_parser::NodeParse;
pub use biome_parser::prelude::*;
use biome_rowan::{AstNode, NodeCache};
use biome_toml_factory::TomlSyntaxFactory;
use biome_toml_syntax::{TomlLanguage, TomlRoot, TomlSyntaxNode};

mod lexer;
mod parser;
mod semantic;
mod syntax;
mod token_source;

pub(crate) type TomlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, TomlLanguage, TomlSyntaxFactory>;

/// Parses `source` as a TOML document.
pub fn parse_toml(source: &str) -> TomlParse {
    let mut cache = NodeCache::default();
    parse_toml_with_cache(source, &mut cache)
}

/// Parses `source` as a TOML document using the provided node cache.
pub fn parse_toml_with_cache(source: &str, cache: &mut NodeCache) -> TomlParse {
    let mut parser = TomlParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = TomlLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    TomlParse::new(green, diagnostics)
}

/// The result of parsing a TOML document.
#[derive(Debug)]
pub struct TomlParse {
    root: TomlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl TomlParse {
    pub fn new(root: TomlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this parse result.
    ///
    /// ```
    /// # use biome_toml_parser::parse_toml;
    /// # use biome_toml_syntax::TomlSyntaxKind;
    /// # use biome_rowan::AstNode;
    /// let parse = parse_toml("key = 'value'");
    ///
    /// assert_eq!(parse.syntax().kind(), TomlSyntaxKind::TOML_ROOT);
    /// assert!(!parse.has_errors());
    /// ```
    pub fn syntax(&self) -> TomlSyntaxNode {
        self.root.clone()
    }

    /// Gets the diagnostics which occurred when parsing.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Takes the diagnostics which occurred when parsing.
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns `true` if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Converts this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> TomlRoot {
        TomlRoot::unwrap_cast(self.syntax())
    }
}

impl From<TomlParse> for AnyParse {
    fn from(parse: TomlParse) -> Self {
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
