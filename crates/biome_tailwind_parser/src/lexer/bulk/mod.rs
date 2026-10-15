//! Lexes a whole class string ahead of parsing, into flat arrays of token starts and kinds.
//!
//! [TailwindLexer] returns each token as the parser asks for it, in the context the parser asks
//! for. Two contexts cover most tokens:
//!
//! - In [TailwindLexContext::Regular], a token depends on its start and on the kind of the
//!   token before it, which picks how a token that starts with a name char ends: a basename,
//!   a value after `-`, a modifier after `/`, a selector after `[` or a value after `(`. Lexing
//!   the whole source in this context, each token after the one before it, predicts the tokens
//!   the parser asks for in this context. A prediction holds where the parser's token before it
//!   leads to the same token, which [previous] classifies.
//! - In [TailwindLexContext::VariantSegment], a token depends on its start alone. The parser
//!   switches to this context at the start of a class and keeps it to the end of each variant,
//!   so this module computes the starts of these tokens from the start of the source, with
//!   bitwise operations on the class bitmaps instead of a walk.
//!
//! [classify] computes bitmaps of character classes 64 bytes at a time with SIMD, among them
//! the token ends of every kind and the variant segment token starts. [BulkTokens::new] then walks
//! the regular tokens from one end to the next, writing their starts and kinds to flat arrays.
//!
//! Tokens with diagnostics are marked [SLOW], so that the token source lexes them with
//! [TailwindLexer] when the parser gets to them, and their diagnostics appear in order and are
//! dropped with a rewind.
//!
//! [TailwindLexer]: super::TailwindLexer
//! [TailwindLexContext::Regular]: crate::token_source::TailwindLexContext::Regular
//! [TailwindLexContext::VariantSegment]: crate::token_source::TailwindLexContext::VariantSegment

mod classify;
#[cfg(test)]
pub(crate) mod tests;

use super::BaseNameStore;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use classify::{Block, classify};
use std::cell::Cell;

/// Mask of the [TailwindSyntaxKind] in a token's kind and flags.
pub(crate) const KIND_MASK: u16 = 0x00FF;
/// The token must be lexed with the lexer when the parser gets to it, because the lexer reports
/// a diagnostic for it.
pub(crate) const SLOW: u16 = 1 << 8;
/// The token is whitespace with a line break.
pub(crate) const LINE_BREAK: u16 = 1 << 9;
/// The shift of the [previous] class that a regular token was lexed after, if it depends on it.
const PREVIOUS_SHIFT: u16 = 12;

const _: () = assert!(TailwindSyntaxKind::__LAST as u16 <= KIND_MASK);

/// The classes of token kinds that lead to different tokens after them in
/// [TailwindLexContext::Regular](crate::token_source::TailwindLexContext::Regular).
pub(crate) mod previous {
    /// A token that starts with whitespace, `(`, `)`, `[` or `]`, which doesn't depend on the
    /// token before it.
    pub(crate) const ANY: u16 = 0;
    pub(crate) const OTHER: u16 = 1;
    pub(crate) const L_BRACK: u16 = 2;
    pub(crate) const L_PAREN: u16 = 3;
    pub(crate) const MINUS: u16 = 4;
    pub(crate) const SLASH: u16 = 5;
}

/// Returns the [previous] class of `kind`.
#[inline(always)]
pub(crate) fn previous_class(kind: TailwindSyntaxKind) -> u16 {
    PREVIOUS_CLASSES[kind as usize]
}

/// The [previous] class of each [TailwindSyntaxKind].
static PREVIOUS_CLASSES: [u16; KIND_MASK as usize + 1] = {
    let mut table = [previous::OTHER; KIND_MASK as usize + 1];
    table[T!['['] as usize] = previous::L_BRACK;
    table[T!['('] as usize] = previous::L_PAREN;
    table[T![-] as usize] = previous::MINUS;
    table[T![/] as usize] = previous::SLASH;
    table
};

/// Returns the [previous] class that the regular token `raw` was lexed after, or
/// [previous::ANY].
#[inline(always)]
pub(crate) fn previous_of(raw: u16) -> u16 {
    raw >> PREVIOUS_SHIFT
}

/// The tokens of a class string, lexed ahead of parsing.
pub(crate) struct BulkTokens {
    blocks: Vec<Block>,
    /// The start of each regular token, followed by the source length.
    starts: Vec<u32>,
    /// The kind and flags of each regular token. The last token is `EOF`.
    kinds: Vec<u16>,
}

/// The largest number of tokens whose buffers [BulkTokens] keeps for the next class string,
/// so that parsing one long class string doesn't keep its memory.
const MAX_KEPT_TOKENS: usize = 1 << 14;

/// The blocks, starts and kinds of a [BulkTokens].
type Buffers = (Vec<Block>, Vec<u32>, Vec<u16>);

thread_local! {
    /// The buffers of the last [BulkTokens] dropped on this thread, so that parsing a class
    /// string doesn't allocate them.
    static BUFFERS: Cell<Option<Buffers>> = const { Cell::new(None) };
}

impl Drop for BulkTokens {
    fn drop(&mut self) {
        let capacity = self.starts.capacity();
        if capacity > 0 && capacity <= MAX_KEPT_TOKENS {
            let buffers = (
                std::mem::take(&mut self.blocks),
                std::mem::take(&mut self.starts),
                std::mem::take(&mut self.kinds),
            );
            // The thread may be ending, in which case the buffers are dropped.
            let _ = BUFFERS.try_with(|cell| cell.set(Some(buffers)));
        }
    }
}

impl BulkTokens {
    /// Lexes `source`, matching dashed basenames against `base_names`.
    pub(crate) fn new(source: &str, base_names: &BaseNameStore) -> Self {
        let (blocks, starts, kinds) = BUFFERS
            .try_with(|cell| cell.take())
            .ok()
            .flatten()
            .unwrap_or_default();
        let mut tokens = Self {
            blocks,
            starts,
            kinds,
        };
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("bmi1")
            && std::arch::is_x86_feature_detected!("bmi2")
            && std::arch::is_x86_feature_detected!("lzcnt")
            && std::arch::is_x86_feature_detected!("popcnt")
        {
            // SAFETY: the CPU supports the features.
            unsafe { tokens.lex_with_bit_instructions(source.as_bytes(), base_names) };
            return tokens;
        }
        tokens.lex_portable(source.as_bytes(), base_names);
        tokens
    }

    /// [Self::lex_portable], compiled with the bit manipulation instructions of x86_64 CPUs
    /// since 2013, which the walk over the bitmaps uses for every token.
    ///
    /// # Safety
    /// The CPU must support AVX2, BMI1, BMI2, LZCNT and POPCNT.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
    unsafe fn lex_with_bit_instructions(&mut self, source: &[u8], base_names: &BaseNameStore) {
        self.lex_portable(source, base_names);
    }

    #[inline(always)]
    fn lex_portable(&mut self, source: &[u8], base_names: &BaseNameStore) {
        classify(source, &mut self.blocks);
        self.lex_regular(source, base_names);
    }

    /// No tokens, so that every token comes from the lexer.
    pub(crate) fn new_empty() -> Self {
        Self {
            blocks: Vec::new(),
            starts: Vec::new(),
            kinds: Vec::new(),
        }
    }

    /// Whether these are [Self::new_empty].
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    /// Returns the kind and flags of regular token `index`.
    #[inline(always)]
    pub(crate) fn get(&self, index: u32) -> Option<u16> {
        self.kinds.get(index as usize).copied()
    }

    /// Returns the start of regular token `index`, or the source length after the last token.
    #[inline(always)]
    pub(crate) fn start(&self, index: u32) -> u32 {
        self.starts[index as usize]
    }

    /// Returns the index of the first regular token starting at `offset`, searching forward from
    /// token `hint` first, as the token source moves forward through the source.
    pub(crate) fn find_from(&self, offset: u32, hint: u32) -> Option<u32> {
        let starts = &self.starts[..self.kinds.len()];
        let hint = hint as usize;
        let mut low = 0;
        if starts.get(hint).is_some_and(|&start| start <= offset) {
            // The token is most often a few tokens after the hint.
            for (index, &start) in starts.iter().enumerate().skip(hint).take(8) {
                if start >= offset {
                    return (start == offset).then_some(index as u32);
                }
            }
            low = hint;
        }
        let index = low + starts[low..].partition_point(|&start| start < offset);
        (starts.get(index) == Some(&offset)).then_some(index as u32)
    }

    /// Returns the kind, flags and end of the token that starts at `offset` in
    /// [TailwindLexContext::VariantSegment](crate::token_source::TailwindLexContext::VariantSegment),
    /// or `None` if the tokens of that context from the start of the source don't start at
    /// `offset`, or if the lexer reports a diagnostic for the token.
    ///
    /// `offset` must be less than the source length.
    #[inline(always)]
    pub(crate) fn segment_token(&self, source: &[u8], offset: usize) -> Option<(u16, usize)> {
        let block = &self.blocks[offset / 64];
        if (block.segment_starts >> (offset % 64)) & 1 == 0 {
            return None;
        }
        let kind = SEGMENT_KINDS[source[offset] as usize];
        if kind == ERROR_TOKEN as u16 {
            return None;
        }
        let end = next_bit(&self.blocks, offset + 1, |block| block.segment_starts);
        if kind == WHITESPACE as u16 {
            return self
                .whitespace_flags(offset, end)
                .map(|flags| (kind | flags, end));
        }
        Some((kind, end))
    }

    /// Returns the flags of the whitespace token `start..end`, or `None` if the lexer reports a
    /// diagnostic for it.
    #[inline(always)]
    fn whitespace_flags(&self, start: usize, end: usize) -> Option<u16> {
        if any_in(&self.blocks, start, end, |block| block.bad_whitespace) {
            return None;
        }
        Some(
            if any_in(&self.blocks, start, end, |block| block.line_break) {
                LINE_BREAK
            } else {
                0
            },
        )
    }

    /// Lexes the regular tokens of `source`, each after the one before it.
    #[inline(always)]
    fn lex_regular(&mut self, source: &[u8], base_names: &BaseNameStore) {
        let len = source.len();
        // Every token takes at least one byte, except for an empty value or modifier, which
        // follows a `-` or `/` of one byte, and `EOF`.
        let capacity = 2 * len + 1;
        self.starts.clear();
        self.kinds.clear();
        self.starts.reserve(capacity + 1);
        self.kinds.reserve(capacity);
        let blocks = &self.blocks[..];
        let starts = &mut self.starts.spare_capacity_mut()[..capacity + 1];
        let kinds = &mut self.kinds.spare_capacity_mut()[..capacity];
        let mut count = 0;
        let mut position = 0;
        // The lexer starts with `EOF` as its current kind.
        let mut previous = previous::OTHER;
        while position < len {
            let byte = source[position];
            let first = REGULAR_FIRST[byte as usize];
            let (raw, end) = if first == WHITESPACE_FIRST {
                let end = next_bit(blocks, position + 1, |block| !block.whitespace);
                let flags = if !any_in(blocks, position, end, |block| {
                    block.bad_whitespace | block.line_break
                }) {
                    0
                } else if any_in(blocks, position, end, |block| block.bad_whitespace) {
                    SLOW
                } else {
                    LINE_BREAK
                };
                (WHITESPACE as u16 | flags, end)
            } else if first & ANY_PREVIOUS_FIRST != 0 {
                (first & KIND_MASK, position + 1)
            } else {
                let (kind, end) = match previous {
                    previous::L_BRACK => {
                        let mut end = next_bit(blocks, position + 1, |block| block.selector_stop);
                        if end < len && source[end] == b'[' {
                            end = nested_selector_end(source, end);
                        }
                        (TW_SELECTOR as u16, end)
                    }
                    previous::L_PAREN => (
                        TW_VALUE as u16,
                        next_bit(blocks, position + 1, |block| block.paren_stop),
                    ),
                    previous::MINUS => {
                        let end = next_bit(blocks, position, |block| block.value_stop);
                        (number_or_value(blocks, position, end), end)
                    }
                    previous::SLASH => {
                        let end = next_bit(blocks, position, |block| block.modifier_stop);
                        let kind = if end == position {
                            ERROR_TOKEN as u16
                        } else {
                            number_or_value(blocks, position, end)
                        };
                        (kind, end)
                    }
                    _ => match first {
                        BASE_FIRST => base(source, blocks, base_names, position),
                        SEGMENT_FIRST => (
                            TW_VARIANT_SEGMENT as u16,
                            next_bit(blocks, position + 1, |block| block.segment_stop),
                        ),
                        ERROR_FIRST => {
                            if position == 0 && source.starts_with(&[0xEF, 0xBB, 0xBF]) {
                                (UNICODE_BOM as u16, 3)
                            } else {
                                (ERROR_TOKEN as u16 | SLOW, position + utf8_len(byte))
                            }
                        }
                        _ => (first, position + 1),
                    },
                };
                (kind | (previous << PREVIOUS_SHIFT), end)
            };
            starts[count].write(position as u32);
            kinds[count].write(raw);
            count += 1;
            previous = PREVIOUS_CLASSES[(raw & KIND_MASK) as usize];
            position = end;
        }
        starts[count].write(len as u32);
        kinds[count].write(EOF as u16);
        starts[count + 1].write(len as u32);
        // SAFETY: the loop and the writes after it initialized the first `count + 1` kinds and
        // `count + 2` starts.
        unsafe {
            self.kinds.set_len(count + 1);
            self.starts.set_len(count + 2);
        }
    }

    /// Returns the offsets in the source of the variant segment token starts.
    #[cfg(test)]
    pub(crate) fn segment_starts(&self, len: usize) -> impl Iterator<Item = usize> + '_ {
        (0..len)
            .filter(|&offset| (self.blocks[offset / 64].segment_starts >> (offset % 64)) & 1 != 0)
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (u32, u32, u16)> + '_ {
        self.kinds
            .iter()
            .enumerate()
            .map(|(index, &kind)| (self.starts[index], self.starts[index + 1], kind))
    }
}

/// Returns the [TailwindSyntaxKind] of a token's kind and flags.
#[inline(always)]
pub(crate) fn kind(raw: u16) -> TailwindSyntaxKind {
    let kind = raw & KIND_MASK;
    debug_assert!(kind <= TailwindSyntaxKind::__LAST as u16);
    // SAFETY: every kind and flags that this module writes holds a `TailwindSyntaxKind` cast to
    // `u16`, and `TailwindSyntaxKind` is a `repr(u16)` enum.
    unsafe { std::mem::transmute::<u16, TailwindSyntaxKind>(kind) }
}

/// Lexes the basename at `start`, as [TailwindLexer](super::TailwindLexer) does.
#[inline(always)]
fn base(source: &[u8], blocks: &[Block], base_names: &BaseNameStore, start: usize) -> (u16, usize) {
    let end = next_bit(blocks, start + 1, |block| block.base_stop);
    if &source[start..end] == b"data" {
        return (DATA_KW as u16, end);
    }
    if source.get(end) != Some(&b'-') {
        return (TW_BASE as u16, end);
    }
    // A dashed basename continues past a '-', so it isn't `data`.
    let end = start + base_names.matcher(&source[start..]).dashed_end(end - start);
    (TW_BASE as u16, end)
}

/// Returns `TW_NUMBER` if `start..end` holds only digits and `.`, and `TW_VALUE` otherwise.
#[inline(always)]
fn number_or_value(blocks: &[Block], start: usize, end: usize) -> u16 {
    if next_bit(blocks, start, |block| !block.numeric) >= end {
        TW_NUMBER as u16
    } else {
        TW_VALUE as u16
    }
}

/// Returns the end of a selector from the `[` at `bracket`, counting nested brackets.
#[cold]
fn nested_selector_end(source: &[u8], bracket: usize) -> usize {
    let mut depth = 0u32;
    let mut position = bracket;
    while let Some(&byte) = source.get(position) {
        match byte {
            b'[' => depth += 1,
            b']' if depth > 0 => depth -= 1,
            b']' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ' => break,
            _ => {}
        }
        position += 1;
    }
    position
}

/// Returns the offset of the first bit of `field` at or after `from`. The fields this is called
/// with have a bit set past the end of the source.
#[inline(always)]
fn next_bit(blocks: &[Block], from: usize, field: impl Fn(&Block) -> u64) -> usize {
    let mut index = from / 64;
    let mut bits = field(&blocks[index]) & (u64::MAX << (from % 64));
    while bits == 0 {
        index += 1;
        bits = field(&blocks[index]);
    }
    index * 64 + bits.trailing_zeros() as usize
}

/// Returns whether `field` has a bit in `from..to`.
#[inline(always)]
fn any_in(blocks: &[Block], from: usize, to: usize, field: impl Fn(&Block) -> u64) -> bool {
    let (first, last) = (from / 64, (to - 1) / 64);
    let low = u64::MAX << (from % 64);
    let high = u64::MAX >> (63 - (to - 1) % 64);
    if first == last {
        return field(&blocks[first]) & low & high != 0;
    }
    field(&blocks[first]) & low != 0
        || blocks[first + 1..last]
            .iter()
            .any(|block| field(block) != 0)
        || field(&blocks[last]) & high != 0
}

/// Returns the length of the UTF-8 char starting with `byte`.
#[inline(always)]
fn utf8_len(byte: u8) -> usize {
    match byte {
        0xF0.. => 4,
        0xE0.. => 3,
        0xC0.. => 2,
        _ => 1,
    }
}

/// [REGULAR_FIRST] of whitespace.
const WHITESPACE_FIRST: u16 = 0x8000;
/// [REGULAR_FIRST] of bytes that start a token of their own after any token. Combined with the
/// token's kind.
const ANY_PREVIOUS_FIRST: u16 = 0x4000;
/// [REGULAR_FIRST] of the bytes that start a basename after a token of class
/// [previous::OTHER].
const BASE_FIRST: u16 = 0x2000;
/// [REGULAR_FIRST] of `*`, which starts a variant segment name after a token of class
/// [previous::OTHER].
const SEGMENT_FIRST: u16 = 0x1000;
/// [REGULAR_FIRST] of the bytes that start an error token after a token of class
/// [previous::OTHER]: the lexer reports them as unexpected.
const ERROR_FIRST: u16 = 0x0800;

/// What the first byte of a regular token says about it: one of the `*_FIRST` constants, or the
/// kind of the one-byte token it starts after a token of class [previous::OTHER].
static REGULAR_FIRST: [u16; 256] = {
    let mut table = [ERROR_FIRST; 256];
    let mut byte = 0;
    while byte < 256 {
        table[byte] = match byte as u8 {
            b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ' => WHITESPACE_FIRST,
            b'(' => ANY_PREVIOUS_FIRST | L_PAREN as u16,
            b')' => ANY_PREVIOUS_FIRST | R_PAREN as u16,
            b'[' => ANY_PREVIOUS_FIRST | T!['['] as u16,
            b']' => ANY_PREVIOUS_FIRST | T![']'] as u16,
            b':' => COLON as u16,
            b'-' => T![-] as u16,
            b'%' => PERCENT as u16,
            b'!' => BANG as u16,
            b'/' => SLASH as u16,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'@' => BASE_FIRST,
            b'*' => SEGMENT_FIRST,
            _ => ERROR_FIRST,
        };
        byte += 1;
    }
    table
};

/// The kind of a token in
/// [TailwindLexContext::VariantSegment](crate::token_source::TailwindLexContext::VariantSegment)
/// by its first byte, where `ERROR_TOKEN` stands for the tokens the lexer reports.
static SEGMENT_KINDS: [u16; 256] = {
    let mut table = [ERROR_TOKEN as u16; 256];
    let mut byte = 0;
    while byte < 256 {
        table[byte] = match byte as u8 {
            b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ' => WHITESPACE as u16,
            b'(' => L_PAREN as u16,
            b')' => R_PAREN as u16,
            b'[' => T!['['] as u16,
            b']' => T![']'] as u16,
            b':' => COLON as u16,
            b'-' => T![-] as u16,
            b'!' => BANG as u16,
            b'/' => SLASH as u16,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'@' | b'*' => {
                TW_VARIANT_SEGMENT as u16
            }
            _ => ERROR_TOKEN as u16,
        };
        byte += 1;
    }
    table
};
