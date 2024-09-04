mod lexer;
mod parser;
mod syntax;
mod token_source;

use crate::parser::{HtmlLosslessTreeSink, HtmlParser};
use crate::syntax::parse_root;
use biome_html_syntax::{HtmlRoot, HtmlSyntaxNode};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};

/// Parses the provided string as HTML program using the provided node cache.
pub fn parse_html_with_cache(source: &str, cache: &mut NodeCache) -> HtmlParse {
    tracing::debug_span!("Parsing phase").in_scope(move || {
        let mut parser = HtmlParser::new(source);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = HtmlLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        HtmlParse::new(green, diagnostics)
    })
}
pub fn parse_html(source: &str) -> HtmlParse {
    let mut cache = NodeCache::default();
    parse_html_with_cache(source, &mut cache)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct HtmlParse {
    root: HtmlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl HtmlParse {
    pub fn new(root: HtmlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_html_parser::parse_html;
    /// # use biome_html_syntax::HtmlSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_html_syntax::HtmlSyntaxKind;
    /// // let parse = parse_html(r#"<html></html>"#);
    ///
    /// // Get the root value
    /// // let root_value = parse.tree().html()?;
    ///
    /// // assert_eq!(root_value.syntax().kind(), HtmlSyntaxKind::HTML_ELEMENT);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> HtmlSyntaxNode {
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
    pub fn tree(&self) -> HtmlRoot {
        HtmlRoot::unwrap_cast(self.syntax())
    }
}

impl From<HtmlParse> for AnyParse {
    fn from(parse: HtmlParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}
