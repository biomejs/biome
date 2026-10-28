//! Lossless, error tolerant TOML 1.1 parser.
//!
//! [`parse_toml`] reports the syntax errors and invalid tokens of a document,
//! following the [TOML 1.1.0 specification](https://toml.io/en/v1.1.0).
//!
//! The parser doesn't check how keys and tables combine. TOML also forbids
//! documents that are syntactically valid but define a key or table more than
//! once, such as a duplicate key, a table defined by two headers, a header for
//! a table that dotted keys already define, dotted keys that extend a table
//! defined by a header, keys added to an inline table, and an array of tables
//! appended to a static array.

// TODO: report the key and table conflicts described above in a syntax rule
// or lint rule once a TOML analyzer exists. Until then, the invalid cases of
// toml-test that only define a key or table twice (for example
// `invalid/key/duplicate-keys-*`, `invalid/table/duplicate-key-*`,
// `invalid/table/append-with-dotted-keys-*`, and
// `invalid/inline-table/overwrite-*`) fail the `toml/toml-test` coverage suite.

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
    /// Creates a parse result. `root` must be the root of a tree, which the
    /// conversion to [AnyParse] relies on.
    pub(crate) fn new(root: TomlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
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
            // SAFETY: `TomlParse` is only created from the root of a parsed tree.
            root.as_send().unwrap(),
            diagnostics,
        )
        .into()
    }
}
