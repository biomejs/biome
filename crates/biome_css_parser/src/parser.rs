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
    pub allow_single_line_comments: bool,
}

impl CssParserOptions {
    pub fn with_allow_single_line_comments(mut self) -> Self {
        self.allow_single_line_comments = true;
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
