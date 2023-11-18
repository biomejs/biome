use crate::lexer::CssReLexContext;
use crate::token_source::CssTokenSource;
use biome_css_syntax::CssSyntaxKind;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;

pub(crate) struct CssParser<'source> {
    context: ParserContext<CssSyntaxKind>,
    source: CssTokenSource<'source>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CssParserOptions {
    pub allow_wrong_line_comments: bool,
}

impl CssParserOptions {
    pub fn with_allow_wrong_line_comments(mut self) -> Self {
        self.allow_wrong_line_comments = true;
        self
    }
}

impl<'source> CssParser<'source> {
    pub fn new(source: &'source str, config: CssParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: CssTokenSource::from_str(source, config),
        }
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    #[allow(dead_code)] //TODO remote this once we actually don't use it
    pub fn re_lex(&mut self, context: CssReLexContext) -> CssSyntaxKind {
        self.source_mut().re_lex(context)
    }

    pub fn finish(self) -> (Vec<Event<CssSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for CssParser<'source> {
    type Kind = CssSyntaxKind;
    type Source = CssTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}
