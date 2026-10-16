//! The token sources of the parser, one over each lexer.
//!
//! [LegacyTokenSource] lexes each token with [TailwindLexer] when the parser asks for it.
//! [SimdTokenSource] lexes the whole class string with [SimdLexer] first, which costs more to
//! start, so short class strings use the legacy token source.
//!
//! [SimdLexer]: crate::lexer::simd::SimdLexer

pub(crate) mod legacy;
pub(crate) mod simd;
#[cfg(test)]
mod tests;

use crate::TailwindParserOptions;
use crate::lexer::legacy::TailwindLexer;
use crate::lexer::{BaseNameStore, TailwindLexContext};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{NthToken, TokenSource, Trivia};
use biome_tailwind_syntax::TailwindSyntaxKind::{self, EOF};
use biome_tailwind_syntax::TextRange;
use legacy::{LegacyTokenSource, LegacyTokenSourceCheckpoint};
use simd::{SimdTokenSource, SimdTokenSourceCheckpoint};

/// The length from which [TailwindTokenSource::from_str] uses [SimdTokenSource]. Below it,
/// classifying the string costs more than lexing it one token at a time.
const MIN_SIMD_LEN: usize = 32;

/// A token source over either lexer.
pub(crate) struct TailwindTokenSource<'source> {
    /// The current token of `inner`, kept here so that reading it doesn't depend on the lexer.
    current: TailwindSyntaxKind,
    current_range: TextRange,
    inner: Inner<'source>,
}

enum Inner<'source> {
    Legacy(LegacyTokenSource<'source>),
    Simd(SimdTokenSource<'source>),
}

/// A position of [TailwindTokenSource] to which it can return.
pub(crate) enum TailwindTokenSourceCheckpoint {
    Legacy(LegacyTokenSourceCheckpoint),
    Simd(SimdTokenSourceCheckpoint),
}

/// Evaluates `$body` with `$inner` bound to the token source in `$inner_source`.
macro_rules! dispatch {
    ($inner_source:expr, $inner:ident => $body:expr) => {
        match $inner_source {
            Inner::Legacy($inner) => $body,
            Inner::Simd($inner) => $body,
        }
    };
}

impl<'source> TailwindTokenSource<'source> {
    /// Creates the token source that lexes `source` fastest.
    pub(crate) fn from_str(source: &'source str, options: TailwindParserOptions<'source>) -> Self {
        if source.len() >= MIN_SIMD_LEN {
            Self::simd(source, options.base_names)
        } else {
            Self::legacy(source, options.base_names)
        }
    }

    /// Creates a token source over [TailwindLexer].
    pub(crate) fn legacy(source: &'source str, base_names: &'source BaseNameStore) -> Self {
        Self::new(Inner::Legacy(LegacyTokenSource::from_str(
            source, base_names,
        )))
    }

    /// Creates a token source over [SimdLexer](crate::lexer::simd::SimdLexer).
    pub(crate) fn simd(source: &'source str, base_names: &'source BaseNameStore) -> Self {
        Self::new(Inner::Simd(SimdTokenSource::from_str(source, base_names)))
    }

    fn new(inner: Inner<'source>) -> Self {
        let mut source = Self {
            current: EOF,
            current_range: TextRange::default(),
            inner,
        };
        source.sync();
        source
    }

    /// Copies the current token of the inner token source.
    #[inline(always)]
    fn sync(&mut self) {
        (self.current, self.current_range) =
            dispatch!(&self.inner, source => (source.current(), source.current_range()));
    }

    pub(crate) fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        match &self.inner {
            Inner::Legacy(source) => TailwindTokenSourceCheckpoint::Legacy(source.checkpoint()),
            Inner::Simd(source) => TailwindTokenSourceCheckpoint::Simd(source.checkpoint()),
        }
    }

    /// Restores the token source to a previous state.
    ///
    /// # Panics
    /// If `checkpoint` comes from the other kind of token source.
    pub(crate) fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        match (&mut self.inner, checkpoint) {
            (Inner::Legacy(source), TailwindTokenSourceCheckpoint::Legacy(checkpoint)) => {
                source.rewind(checkpoint)
            }
            (Inner::Simd(source), TailwindTokenSourceCheckpoint::Simd(checkpoint)) => {
                source.rewind(checkpoint)
            }
            _ => unreachable!("a checkpoint of the other token source"),
        }
        self.sync();
    }

    /// Lexes the current token again in `context`, as if no token came before it.
    pub(crate) fn re_lex_current_in_context(
        &mut self,
        context: TailwindLexContext,
    ) -> TailwindSyntaxKind {
        dispatch!(&mut self.inner, source => source.re_lex_current_in_context(context));
        self.sync();
        self.current
    }

    /// Returns whether a `:` follows the start of the current token before the next whitespace.
    pub(crate) fn has_colon_before_whitespace(&self) -> bool {
        dispatch!(&self.inner, source => source.has_colon_before_whitespace())
    }
}

impl TokenSource for TailwindTokenSource<'_> {
    type Kind = TailwindSyntaxKind;

    #[inline(always)]
    fn current(&self) -> Self::Kind {
        self.current
    }

    #[inline(always)]
    fn current_range(&self) -> TextRange {
        self.current_range
    }

    fn text(&self) -> &str {
        dispatch!(&self.inner, source => source.text())
    }

    fn has_preceding_line_break(&self) -> bool {
        dispatch!(&self.inner, source => source.has_preceding_line_break())
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(TailwindLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TailwindLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        dispatch!(self.inner, source => source.finish())
    }
}

impl BumpWithContext for TailwindTokenSource<'_> {
    type Context = TailwindLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        dispatch!(&mut self.inner, source => source.bump_with_context(context));
        self.sync();
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        dispatch!(&mut self.inner, source => source.skip_as_trivia_with_context(context));
        self.sync();
    }
}

impl<'source> NthToken<TailwindLexer<'source>> for TailwindTokenSource<'source> {
    fn nth(&mut self, n: usize) -> TailwindSyntaxKind {
        match &mut self.inner {
            _ if n == 0 => self.current,
            Inner::Legacy(source) => NthToken::nth(source, n),
            Inner::Simd(source) => source.lookahead(n).map_or(EOF, |token| token.kind),
        }
    }

    fn nth_range(&mut self, n: usize) -> Option<TextRange> {
        match &mut self.inner {
            _ if n == 0 => Some(self.current_range),
            Inner::Legacy(source) => source.nth_range(n),
            Inner::Simd(source) => source.lookahead(n).map(|token| token.range),
        }
    }

    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        match &mut self.inner {
            _ if n == 0 => self.has_preceding_line_break(),
            Inner::Legacy(source) => source.has_nth_preceding_line_break(n),
            Inner::Simd(source) => source
                .lookahead(n)
                .is_some_and(|token| token.has_preceding_line_break),
        }
    }

    /// The Tailwind lexers never report whitespace before a token.
    fn has_nth_preceding_whitespace(&mut self, _n: usize) -> bool {
        false
    }
}
