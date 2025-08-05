#![deny(clippy::use_self)]

mod lexer;
mod parser;
mod syntax;
mod token_source;

use crate::parser::{TailwindLosslessTreeSink, TailwindParser};
use crate::syntax::parse_root;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{AstNode, NodeCache};
use biome_tailwind_syntax::{TailwindSyntaxNode, TwRoot};

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

// impl From<TailwindParse> for AnyParse {
//     fn from(parse: TailwindParse) -> Self {
//         let root = parse.syntax();
//         let diagnostics = parse.into_diagnostics();
//         Self::new(
//             // SAFETY: the parser should always return a root node
//             root.as_send().unwrap(),
//             diagnostics,
//         )
//     }
// }
