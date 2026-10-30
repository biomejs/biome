//! The token source of the parser: an index into the atoms that [SimdLexer] wrote before
//! parsing.

#[cfg(test)]
mod tests;

use crate::lexer::BaseNameStore;
use crate::lexer::simd::SimdLexer;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::token_source::{TokenSource, Trivia};
use biome_rowan::TriviaPieceKind;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, EOF, WHITESPACE};
use biome_tailwind_syntax::{TextRange, TextSize};
use biome_unicode_table::{Dispatch, lookup_byte};

/// Token source for the parser.
///
/// The current token is one atom of the lexer's stream. Bumping it moves to the next atom;
/// the parser merges several atoms into one token of the tree by bumping them together with
/// [Self::advance]. A checkpoint is an index, so a rewind never lexes anything again.
pub(crate) struct TailwindTokenSource<'source> {
    lexer: SimdLexer<'source>,
    /// The index of the current atom.
    position: usize,
    /// Trivia that the parser skipped.
    trivia_list: Vec<Trivia>,
}

/// A position of [TailwindTokenSource] to which it can return.
#[derive(Debug)]
pub(crate) struct TailwindTokenSourceCheckpoint {
    position: usize,
    trivia_len: u32,
}

impl<'source> TailwindTokenSource<'source> {
    /// Lexes `source` and creates a token source at its first atom.
    pub(crate) fn from_str(source: &'source str, base_names: &'source BaseNameStore) -> Self {
        Self {
            lexer: SimdLexer::new(source, base_names),
            position: 0,
            trivia_list: vec![],
        }
    }

    /// Returns the index of the current atom.
    #[inline(always)]
    pub(crate) fn position_index(&self) -> usize {
        self.position
    }

    /// Returns the kind of the atom `n` atoms after the current one, or `EOF` past the end.
    #[inline(always)]
    pub(crate) fn nth_kind(&self, n: usize) -> TailwindSyntaxKind {
        self.lexer.kind(self.position + n)
    }

    /// Returns the first byte of the atom `n` atoms after the current one, or `None` past
    /// the end.
    #[inline(always)]
    pub(crate) fn nth_byte(&self, n: usize) -> Option<u8> {
        self.lexer.byte(self.position + n)
    }

    /// Returns the [Dispatch] of the first byte of the atom `n` atoms after the current one,
    /// or `None` past the end.
    #[inline(always)]
    pub(crate) fn nth_dispatch(&self, n: usize) -> Option<Dispatch> {
        self.nth_byte(n).map(lookup_byte)
    }

    /// Returns the text of the atom `n` atoms after the current one.
    #[inline(always)]
    pub(crate) fn nth_text(&self, n: usize) -> &'source str {
        self.lexer.text(self.position + n)
    }

    /// Returns the start of the atom `n` atoms after the current one, or the source length
    /// past the end.
    #[inline(always)]
    pub(crate) fn nth_start(&self, n: usize) -> usize {
        self.lexer.start(self.position + n)
    }

    /// Returns how many atoms after the current one start before `offset`, which must be a
    /// atom boundary at or after the current atom.
    #[inline(always)]
    pub(crate) fn atoms_before(&self, offset: usize) -> usize {
        self.lexer.index_at(offset, self.position) - self.position
    }

    /// Returns the end of the basename that starts at `start` and runs to `end`, before a `-`,
    /// continued over each `-` that a dashed basename continues over.
    #[inline(always)]
    pub(crate) fn dashed_base_end(&self, start: usize, end: usize) -> usize {
        self.lexer.dashed_base_end(start, end)
    }

    /// Moves `count` atoms forward.
    #[inline(always)]
    pub(crate) fn advance(&mut self, count: usize) {
        self.position += count;
    }

    /// Skips the current atom as trivia of the token before it.
    pub(crate) fn skip_as_trailing_trivia(&mut self, kind: TriviaPieceKind) {
        self.trivia_list
            .push(Trivia::new(kind, self.current_range(), true));
        self.position += 1;
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub(crate) fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        TailwindTokenSourceCheckpoint {
            position: self.position,
            trivia_len: self.trivia_list.len() as u32,
        }
    }

    /// Restores the token source to a previous state.
    pub(crate) fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.position = checkpoint.position;
    }

    /// Returns whether a `:` follows the start of the current atom before the next whitespace.
    pub(crate) fn has_colon_before_whitespace(&self) -> bool {
        self.lexer
            .has_colon_before_whitespace(self.lexer.start(self.position))
    }
}

impl TokenSource for TailwindTokenSource<'_> {
    type Kind = TailwindSyntaxKind;

    #[inline(always)]
    fn current(&self) -> Self::Kind {
        self.lexer.kind(self.position)
    }

    #[inline(always)]
    fn current_range(&self) -> TextRange {
        TextRange::new(
            TextSize::from(self.lexer.start(self.position) as u32),
            TextSize::from(self.lexer.start(self.position + 1) as u32),
        )
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.position > 0
            && self.lexer.kind(self.position - 1) == WHITESPACE
            && self.lexer.has_line_break(
                self.lexer.start(self.position - 1),
                self.lexer.start(self.position),
            )
    }

    #[inline(always)]
    fn bump(&mut self) {
        if self.current() != EOF {
            self.position += 1;
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));
            self.position += 1;
        }
    }

    fn finish(mut self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        let diagnostics = self.lexer.take_diagnostics();
        (std::mem::take(&mut self.trivia_list), diagnostics)
    }
}
