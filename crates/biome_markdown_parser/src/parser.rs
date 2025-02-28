use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_parser::{diagnostic::merge_diagnostics, ParserContextCheckpoint};

use crate::token_source::{MarkdownTokenSource, MarkdownTokenSourceCheckpoint};

pub(crate) struct MarkdownParser<'source> {
    context: ParserContext<MarkdownSyntaxKind>,
    source: MarkdownTokenSource<'source>,
}

impl<'source> MarkdownParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: MarkdownTokenSource::from_str(source),
        }
    }

    pub fn checkpoint(&self) -> MarkdownParserCheckpoint {
        MarkdownParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
        }
    }

    pub fn before_whitespace_count(&self) -> usize {
        self.source.before_whitespace_count()
    }

    pub fn rewind(&mut self, checkpoint: MarkdownParserCheckpoint) {
        let MarkdownParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<MarkdownSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for MarkdownParser<'source> {
    type Kind = MarkdownSyntaxKind;
    type Source = MarkdownTokenSource<'source>;

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

pub struct MarkdownParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: MarkdownTokenSourceCheckpoint,
}
