//! The token source of the parser.
//!
//! The parser reads its tokens from [SimdTokenSource]. Tests can also run it over
//! `LegacyTokenSource`, the reference implementation, and compare the results.

#[cfg(test)]
pub(crate) mod legacy;
pub(crate) mod simd;
#[cfg(test)]
mod tests;

use crate::TailwindParserOptions;
use crate::lexer::TailwindLexContext;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{TokenSource, Trivia};
use biome_tailwind_syntax::TailwindSyntaxKind::{self, EOF};
use biome_tailwind_syntax::TextRange;
#[cfg(test)]
use legacy::{LegacyTokenSource, LegacyTokenSourceCheckpoint};
use simd::{SimdTokenSource, SimdTokenSourceCheckpoint};

pub(crate) enum TailwindTokenSource<'source> {
    Simd(SimdTokenSource<'source>),
    #[cfg(test)]
    Legacy(LegacyTokenSource<'source>),
}

/// A position of [TailwindTokenSource] to which it can return.
pub(crate) enum TailwindTokenSourceCheckpoint {
    Simd(SimdTokenSourceCheckpoint),
    #[cfg(test)]
    Legacy(LegacyTokenSourceCheckpoint),
}

/// Evaluates `$body` with `$inner` bound to the token source inside `$source`.
macro_rules! dispatch {
    ($source:expr, $inner:ident => $body:expr) => {
        match $source {
            TailwindTokenSource::Simd($inner) => $body,
            #[cfg(test)]
            TailwindTokenSource::Legacy($inner) => $body,
        }
    };
}

impl<'source> TailwindTokenSource<'source> {
    pub(crate) fn from_str(source: &'source str, options: TailwindParserOptions<'source>) -> Self {
        Self::Simd(SimdTokenSource::from_str(source, options.base_names))
    }

    pub(crate) fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        match self {
            Self::Simd(source) => TailwindTokenSourceCheckpoint::Simd(source.checkpoint()),
            #[cfg(test)]
            Self::Legacy(source) => TailwindTokenSourceCheckpoint::Legacy(source.checkpoint()),
        }
    }

    /// Restores the token source to a previous state.
    pub(crate) fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        match (self, checkpoint) {
            (Self::Simd(source), TailwindTokenSourceCheckpoint::Simd(checkpoint)) => {
                source.rewind(checkpoint)
            }
            #[cfg(test)]
            (Self::Legacy(source), TailwindTokenSourceCheckpoint::Legacy(checkpoint)) => {
                source.rewind(checkpoint)
            }
            #[cfg(test)]
            _ => unreachable!("a checkpoint of the other token source"),
        }
    }

    /// Lexes the current token again in `context`, as if no token came before it.
    pub(crate) fn re_lex_current_in_context(
        &mut self,
        context: TailwindLexContext,
    ) -> TailwindSyntaxKind {
        dispatch!(self, source => source.re_lex_current_in_context(context))
    }

    /// Returns whether a `:` follows the start of the current token before the next whitespace.
    pub(crate) fn has_colon_before_whitespace(&self) -> bool {
        dispatch!(self, source => source.has_colon_before_whitespace())
    }

    /// Returns the kind and range of the `n`th non-trivia token after the current one, lexed in
    /// [TailwindLexContext::Regular], if there is one.
    #[cfg(test)]
    pub(crate) fn nth_token(&mut self, n: usize) -> Option<(TailwindSyntaxKind, TextRange)> {
        use biome_parser::token_source::NthToken;
        match self {
            Self::Simd(source) => source.lookahead(n).map(|token| (token.kind, token.range)),
            Self::Legacy(source) => source.nth_range(n).map(|range| (source.nth(n), range)),
        }
    }

    /// Returns the kind of the `n`th non-trivia token after the current one, lexed in
    /// [TailwindLexContext::Regular], or `EOF` past the end.
    pub(crate) fn nth(&mut self, n: usize) -> TailwindSyntaxKind {
        if n == 0 {
            return self.current();
        }
        match self {
            Self::Simd(source) => source.lookahead(n).map_or(EOF, |token| token.kind),
            #[cfg(test)]
            Self::Legacy(source) => biome_parser::token_source::NthToken::nth(source, n),
        }
    }
}

impl TokenSource for TailwindTokenSource<'_> {
    type Kind = TailwindSyntaxKind;

    #[inline(always)]
    fn current(&self) -> Self::Kind {
        dispatch!(self, source => source.current())
    }

    #[inline(always)]
    fn current_range(&self) -> TextRange {
        dispatch!(self, source => source.current_range())
    }

    fn text(&self) -> &str {
        dispatch!(self, source => source.text())
    }

    fn has_preceding_line_break(&self) -> bool {
        dispatch!(self, source => source.has_preceding_line_break())
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(TailwindLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TailwindLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        dispatch!(self, source => source.finish())
    }
}

impl BumpWithContext for TailwindTokenSource<'_> {
    type Context = TailwindLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        dispatch!(self, source => source.bump_with_context(context))
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        dispatch!(self, source => source.skip_as_trivia_with_context(context))
    }
}
