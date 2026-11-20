use crate::TailwindParserOptions;
use crate::syntax::css_token;
use crate::token_source::{TailwindTokenSource, TailwindTokenSourceCheckpoint};
use biome_parser::diagnostic::expected_token;
use biome_parser::diagnostic::{ParseDiagnostic, merge_diagnostics};
use biome_parser::event::Event;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext};
use biome_parser::{ParserContextCheckpoint, prelude::*};
use biome_rowan::TriviaPieceKind;
use biome_tailwind_factory::TailwindSyntaxFactory;
use biome_tailwind_syntax::TailwindSyntaxKind::{CSS_WHITESPACE, EOF};
use biome_tailwind_syntax::{TailwindLanguage, TailwindSyntaxKind, TextSize};
use std::cell::Cell;

pub(crate) type TailwindLosslessTreeSink<'source> =
    LosslessTreeSink<'source, TailwindLanguage, TailwindSyntaxFactory>;

pub(crate) struct TailwindParser<'source> {
    context: ParserContext<TailwindSyntaxKind>,
    source: TailwindTokenSource<'source>,
    /// The last result of [Self::css_token]: the atom index it was asked for, and the kind
    /// and length in atoms of the CSS token there. The atoms never change, so it stays
    /// valid across rewinds.
    css_token: Cell<(usize, TailwindSyntaxKind, usize)>,
}

impl<'source> TailwindParser<'source> {
    pub fn new(source: &'source str, options: TailwindParserOptions<'source>) -> Self {
        Self {
            context: ParserContext::default(),
            source: TailwindTokenSource::from_str(source, options.base_names),
            css_token: Cell::new((usize::MAX, EOF, 0)),
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

    /// Returns the kind and length in atoms of the CSS token at the current atom.
    pub fn css_token(&self) -> (TailwindSyntaxKind, usize) {
        let position = self.source.position_index();
        let (cached, kind, atoms) = self.css_token.get();
        if cached == position {
            return (kind, atoms);
        }
        let (kind, atoms) = css_token(&self.source, 0);
        self.css_token.set((position, kind, atoms));
        (kind, atoms)
    }

    /// Bumps the current atom and the `count - 1` atoms after it as one token of `kind`, or
    /// adds an empty token of `kind` if `count` is zero.
    pub fn bump_atoms(&mut self, kind: TailwindSyntaxKind, count: usize) {
        let end = TextSize::from(self.source.nth_start(count) as u32);
        self.context.push_token(kind, end);
        self.source.advance(count);
    }

    /// Bumps the current atom as one token of `kind`, then skips the `_`s after it, which
    /// Tailwind decodes to whitespace inside a CSS value, as trivia.
    pub fn bump_css(&mut self, kind: TailwindSyntaxKind) {
        self.bump_css_atoms(kind, 1);
    }

    /// Like [Self::bump_atoms], then skips the `_`s after the token, which Tailwind decodes to
    /// whitespace inside a CSS value, as trivia.
    pub fn bump_css_atoms(&mut self, kind: TailwindSyntaxKind, count: usize) {
        self.bump_atoms(kind, count);
        self.skip_css_whitespace();
    }

    /// Skips the `_`s at the current atom, which Tailwind decodes to whitespace inside a CSS
    /// value, as trivia of the token before them.
    pub fn skip_css_whitespace(&mut self) {
        while self.source.nth_kind(0) == CSS_WHITESPACE {
            self.source
                .skip_as_trailing_trivia(TriviaPieceKind::Whitespace);
        }
    }

    /// Bumps the current atom if it is of `kind`, then skips the `_`s after it as in
    /// [Self::bump_css]. Adds a diagnostic if it isn't.
    pub fn expect_css(&mut self, kind: TailwindSyntaxKind) -> bool {
        if self.at(kind) {
            self.bump_css(kind);
            true
        } else {
            self.error(expected_token(kind));
            false
        }
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
