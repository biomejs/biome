//! The token stream that the parser reads: a gap buffer, so that splicing tokens in where the
//! parser is costs what the parser moved since the last splice rather than the rest of the
//! stream.

/// A token of the stream: its kind and flags, and its range.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Token {
    pub(crate) word: u32,
    pub(crate) start: u32,
    pub(crate) end: u32,
}

/// Tokens in a gap buffer, addressed by their index in the stream.
///
/// The gap is where the last splice happened. The token source only reads ahead of a position at
/// or after the gap, which [Self::keep_gap_before] ensures, so [Self::ahead] needn't check on
/// which side of the gap a token is.
pub(super) struct Stream {
    tokens: Vec<Token>,
    gap_start: usize,
    gap_end: usize,
}

impl Stream {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self {
            gap_start: 0,
            gap_end: 0,
            tokens,
        }
    }

    #[inline(always)]
    fn gap_len(&self) -> usize {
        self.gap_end - self.gap_start
    }

    #[inline(always)]
    fn physical(&self, index: usize) -> usize {
        if index < self.gap_start {
            index
        } else {
            index + self.gap_len()
        }
    }

    pub(super) fn len(&self) -> usize {
        self.tokens.len() - self.gap_len()
    }

    /// Returns the token at `index`.
    #[inline]
    pub(super) fn get(&self, index: usize) -> Token {
        self.tokens[self.physical(index)]
    }

    #[inline]
    pub(super) fn get_mut(&mut self, index: usize) -> &mut Token {
        let index = self.physical(index);
        &mut self.tokens[index]
    }

    /// Returns the token at `index`, which is at or after the gap.
    #[inline(always)]
    pub(super) fn ahead(&self, index: usize) -> &Token {
        debug_assert!(index >= self.gap_start);
        &self.tokens[index + self.gap_len()]
    }

    /// Returns the token at `index`, which is at or after the gap, if the stream has one there.
    #[inline(always)]
    pub(super) fn get_ahead(&self, index: usize) -> Option<&Token> {
        debug_assert!(index >= self.gap_start);
        self.tokens.get(index + self.gap_len())
    }

    #[inline(always)]
    pub(super) fn ahead_mut(&mut self, index: usize) -> &mut Token {
        debug_assert!(index >= self.gap_start);
        let gap_len = self.gap_len();
        &mut self.tokens[index + gap_len]
    }

    /// Moves the gap to `index`, if it's after it.
    #[inline]
    pub(super) fn keep_gap_before(&mut self, index: usize) {
        if self.gap_start > index {
            self.move_gap(index);
        }
    }

    fn move_gap(&mut self, index: usize) {
        if index < self.gap_start {
            let moved = self.gap_start - index;
            self.tokens
                .copy_within(index..self.gap_start, self.gap_end - moved);
            self.gap_start = index;
            self.gap_end -= moved;
        } else if index > self.gap_start {
            let moved = index - self.gap_start;
            self.tokens
                .copy_within(self.gap_end..self.gap_end + moved, self.gap_start);
            self.gap_start = index;
            self.gap_end += moved;
        }
    }

    /// Puts `inserted` in place of the `removed` tokens from `index`, returning those, and leaves
    /// the gap at `index`.
    pub(super) fn splice(
        &mut self,
        index: usize,
        removed: usize,
        inserted: &[Token],
    ) -> Vec<Token> {
        self.move_gap(index);
        let taken = self.tokens[self.gap_end..self.gap_end + removed].to_vec();
        self.gap_end += removed;
        if self.gap_len() < inserted.len() {
            let grow = inserted.len().max(self.tokens.len() / 64 + 16);
            self.tokens.splice(
                self.gap_end..self.gap_end,
                std::iter::repeat_n(Token::default(), grow),
            );
            self.gap_end += grow;
        }
        self.gap_end -= inserted.len();
        self.tokens[self.gap_end..self.gap_end + inserted.len()].copy_from_slice(inserted);
        taken
    }
}
