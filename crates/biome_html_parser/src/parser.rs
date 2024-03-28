use crate::token_source::HtmlTokenSource;
use biome_html_factory::HtmlSyntaxFactory;
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxKind};
use biome_parser::diagnostic::{merge_diagnostics, ParseDiagnostic};
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext};

pub(crate) type HtmlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, HtmlLanguage, HtmlSyntaxFactory>;

pub(crate) struct HtmlParser<'source> {
    context: ParserContext<HtmlSyntaxKind>,
    source: HtmlTokenSource<'source>,
}

impl<'source> HtmlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: HtmlTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<HtmlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'src> Parser for HtmlParser<'src> {
    type Kind = HtmlSyntaxKind;
    type Source = HtmlTokenSource<'src>;

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
