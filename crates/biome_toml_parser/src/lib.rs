//! Lossless parser for TOML 1.1 documents.

#![deny(clippy::use_self)]

use biome_parser::{
    AnyParse, NodeParse, diagnostic::merge_diagnostics, prelude::ParseDiagnostic,
    tree_sink::LosslessTreeSink,
};
use biome_rowan::{AstNode, NodeCache};
use biome_toml_factory::TomlSyntaxFactory;
use biome_toml_syntax::{TomlLanguage, TomlRoot, TomlSyntaxNode};
use parser::TomlParser;
use syntax::parse_root;

mod definitions;
mod lexer;
mod parser;
mod syntax;
mod token_source;

type TomlLosslessTreeSink<'source> = LosslessTreeSink<'source, TomlLanguage, TomlSyntaxFactory>;

/// Parses a TOML document.
pub fn parse_toml(source: &str) -> TomlParse {
    parse_toml_with_cache(source, &mut NodeCache::default())
}

/// Parses a TOML document using `cache` to reuse syntax nodes.
pub fn parse_toml_with_cache(source: &str, cache: &mut NodeCache) -> TomlParse {
    let mut parser = TomlParser::new(source);
    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();
    let mut tree_sink = TomlLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (root, diagnostics) = tree_sink.finish();
    let definition_diagnostics =
        definitions::validate_definitions(&TomlRoot::unwrap_cast(root.clone()));
    let diagnostics = merge_diagnostics(diagnostics, definition_diagnostics);

    TomlParse::new(root, diagnostics)
}

/// The syntax tree and diagnostics produced for a TOML document.
#[derive(Debug)]
pub struct TomlParse {
    root: TomlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl TomlParse {
    fn new(root: TomlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// Returns the untyped syntax tree.
    pub fn syntax(&self) -> TomlSyntaxNode {
        self.root.clone()
    }

    /// Returns the diagnostics emitted while parsing.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Consumes the parse result and returns its diagnostics.
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns whether the parser emitted an error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Returns the typed root node.
    pub fn tree(&self) -> TomlRoot {
        TomlRoot::unwrap_cast(self.syntax())
    }
}

impl From<TomlParse> for AnyParse {
    fn from(parse: TomlParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        NodeParse::new(root.as_send().unwrap(), diagnostics).into()
    }
}
