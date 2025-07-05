use biome_astro_syntax::{AstroFileSource, AstroLanguage, AstroSyntaxNode};
use biome_parser::prelude::*;
use biome_rowan::{AstNode, NodeOrToken};

mod lexer;
mod parser;
mod prelude;
mod token_source;

pub use lexer::*;
pub use parser::*;
pub use token_source::*;

use crate::prelude::*;

pub fn parse_astro(source: &str, file_source: AstroFileSource) -> AstroParse {
    let mut cache = ParsedSyntaxCache::default();
    parse_astro_with_cache(source, file_source, &mut cache)
}

pub fn parse_astro_with_cache(
    source: &str,
    file_source: AstroFileSource,
    cache: &mut ParsedSyntaxCache,
) -> AstroParse {
    let mut source = TokenSource::new(source, file_source);
    let mut parser = AstroParser::new(&mut source);

    parse_root(&mut parser);

    let (green, diagnostics) = parser.finish();
    let root = AstroSyntaxNode::new_root(green);

    AstroParse::new(root, diagnostics, source.trivia())
}

#[derive(Debug, Clone)]
pub struct AstroParse {
    root: AstroSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
    trivia: Vec<SyntaxTrivia>,
}

impl AstroParse {
    pub fn new(
        root: AstroSyntaxNode,
        diagnostics: Vec<ParseDiagnostic>,
        trivia: Vec<SyntaxTrivia>,
    ) -> Self {
        Self {
            root,
            diagnostics,
            trivia,
        }
    }

    pub fn syntax(&self) -> AstroSyntaxNode {
        self.root.clone()
    }

    pub fn root(&self) -> biome_astro_syntax::AstroRoot {
        biome_astro_syntax::AstroRoot::cast(self.root.clone()).unwrap()
    }

    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    pub fn trivia(&self) -> &[SyntaxTrivia] {
        &self.trivia
    }

    /// Get the diagnostics which represent parsing errors
    pub fn errors(&self) -> impl Iterator<Item = &ParseDiagnostic> {
        self.diagnostics.iter().filter(|d| d.is_error())
    }

    /// Returns [true] if the parser encountered some errors during the parsing phase
    pub fn has_parser_errors(&self) -> bool {
        self.errors().next().is_some()
    }

    /// Consumes `self` and returns the green node
    #[allow(unused)]
    pub fn green(self) -> biome_rowan::GreenNode {
        self.root.green().clone()
    }

    /// Consumes `self` and returns the syntax node
    #[allow(unused)]
    pub fn tree(self) -> AstroSyntaxNode {
        self.root
    }
}

impl From<AstroParse> for biome_rowan::SyntaxNode<AstroLanguage> {
    fn from(parse: AstroParse) -> Self {
        parse.syntax()
    }
}

impl From<AstroParse> for biome_astro_syntax::AstroRoot {
    fn from(parse: AstroParse) -> Self {
        parse.root()
    }
}