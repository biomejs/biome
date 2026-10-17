use super::token_source::{LegacyTokenSource, LegacyTokenSourceCheckpoint};
use biome_parser::diagnostic::{ParseDiagnostic, merge_diagnostics};
use biome_parser::event::Event;
use biome_parser::token_source::NthToken;
use biome_parser::{Parser, ParserContext, TokenSet};
use biome_parser::{ParserContextCheckpoint, prelude::*};
use biome_tailwind_syntax::TailwindSyntaxKind;

pub(crate) struct LegacyParser<'source> {
    context: ParserContext<TailwindSyntaxKind>,
    source: LegacyTokenSource<'source>,
}

impl<'source> LegacyParser<'source> {
    pub(crate) fn new(source: LegacyTokenSource<'source>) -> Self {
        Self {
            context: ParserContext::default(),
            source,
        }
    }

    pub(crate) fn finish(
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

    pub(crate) fn checkpoint(&self) -> LegacyParserCheckpoint {
        LegacyParserCheckpoint {
            parser: self.context.checkpoint(),
            token_source: self.source.checkpoint(),
        }
    }

    pub(crate) fn rewind(&mut self, checkpoint: LegacyParserCheckpoint) {
        self.context.rewind(checkpoint.parser);
        self.source.rewind(checkpoint.token_source);
    }

    /// Whether the `n`th non-trivia token after the current one, lexed in
    /// [TailwindLexContext::Regular](crate::lexer::TailwindLexContext::Regular), is of `kind`.
    pub(crate) fn nth_at(&mut self, n: usize, kind: TailwindSyntaxKind) -> bool {
        self.source.nth(n) == kind
    }

    /// Whether the `n`th non-trivia token after the current one, lexed in
    /// [TailwindLexContext::Regular](crate::lexer::TailwindLexContext::Regular), is of one of
    /// `kinds`.
    pub(crate) fn nth_at_ts(&mut self, n: usize, kinds: TokenSet<TailwindSyntaxKind>) -> bool {
        kinds.contains(self.source.nth(n))
    }
}

impl<'src> Parser for LegacyParser<'src> {
    type Kind = TailwindSyntaxKind;
    type Source = LegacyTokenSource<'src>;

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

pub(crate) struct LegacyParserCheckpoint {
    parser: ParserContextCheckpoint,
    token_source: LegacyTokenSourceCheckpoint,
}
