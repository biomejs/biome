use crate::token_source::{TailwindTokenSource, TailwindTokenSourceCheckpoint};
use biome_parser::diagnostic::{ParseDiagnostic, merge_diagnostics};
use biome_parser::event::Event;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext};
use biome_parser::{ParserContextCheckpoint, prelude::*};
use biome_tailwind_factory::TailwindSyntaxFactory;
use biome_tailwind_syntax::{TailwindLanguage, TailwindSyntaxKind};

pub(crate) type TailwindLosslessTreeSink<'source> =
    LosslessTreeSink<'source, TailwindLanguage, TailwindSyntaxFactory>;

pub(crate) struct TailwindParser<'source> {
    context: ParserContext<TailwindSyntaxKind>,
    source: TailwindTokenSource<'source>,
}

impl<'source> TailwindParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: TailwindTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<TailwindSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }

    pub fn checkpoint(&self) -> TailwindParserCheckpoint {
        TailwindParserCheckpoint {
            parser: self.context.checkpoint(),
            token_source: self.source.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: TailwindParserCheckpoint) {
        self.context.rewind(checkpoint.parser);
        self.source.rewind(checkpoint.token_source);
    }
}

impl<'src> Parser for TailwindParser<'src> {
    type Kind = TailwindSyntaxKind;
    type Source = TailwindTokenSource<'src>;

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

pub struct TailwindParserCheckpoint {
    parser: ParserContextCheckpoint,
    token_source: TailwindTokenSourceCheckpoint,
}
