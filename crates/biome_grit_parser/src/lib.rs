mod lexer;
mod parser;
mod token_source;

use biome_grit_factory::GritSyntaxFactory;
use biome_grit_syntax::{GritLanguage, GritRoot, GritSyntaxNode};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_rowan::{AstNode, NodeCache};
use parser::{parse_root, GritParser};

pub(crate) type GritLosslessTreeSink<'source> =
    LosslessTreeSink<'source, GritLanguage, GritSyntaxFactory>;

pub fn parse_grit(source: &str) -> GritParse {
    let mut cache = NodeCache::default();
    parse_grit_with_cache(source, &mut cache)
}

/// Parses the provided string as a GritQL pattern using the provided node cache.
pub fn parse_grit_with_cache(source: &str, cache: &mut NodeCache) -> GritParse {
    tracing::debug_span!("parse").in_scope(move || {
        let mut parser = GritParser::new(source);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = GritLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        GritParse::new(green, diagnostics)
    })
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
    /// # use biome_grit_syntax::GritSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_grit_syntax::GritSyntaxKind;
    /// use biome_grit_parser::GritParserOptions;
    /// let parse = parse_grit(r#"`console.log($message)`"#);
    ///
    /// // Get the root value
    /// let root_value = parse.tree().value()?;
    ///
    /// assert_eq!(root_value.syntax().kind(), GritSyntaxKind::GRIT_ARRAY_VALUE);
    ///
    /// # Ok(())
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
