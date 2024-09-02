use biome_markdown_factory::DemoSyntaxFactory;
use biome_markdown_syntax::{DemoLanguage, DemoSyntaxNode, Root};
use biome_parser::{prelude::ParseDiagnostic, tree_sink::LosslessTreeSink};
use biome_rowan::{AstNode, NodeCache};
use parser::DemoParser;
use syntax::parse_root;

mod lexer;
mod parser;
mod token_source;
mod syntax;

pub(crate) type DemoLosslessTreeSink<'source> =
    LosslessTreeSink<'source, DemoLanguage, DemoSyntaxFactory>;

pub fn parse_demo(source: &str) -> DemoParse {
    let mut cache = NodeCache::default();
    parse_demo_with_cache(source, &mut cache)
}

pub fn parse_demo_with_cache(source: &str, cache: &mut NodeCache) -> DemoParse {
    tracing::debug_span!("Parsing phase").in_scope(move || {
        let mut parser = DemoParser::new(source);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = DemoLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        DemoParse::new(green, diagnostics)
    })
}



/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct DemoParse {
    root: DemoSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl DemoParse {
    pub fn new(root: DemoSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> DemoParse {
        DemoParse { root, diagnostics }
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
    pub fn syntax(&self) -> DemoSyntaxNode {
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
    pub fn tree(&self) -> Root {
        Root::unwrap_cast(self.syntax())
    }
}
