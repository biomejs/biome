//! The lexer of Tailwind class strings, which runs once before parsing.
//!
//! [classify] computes bitmaps of character classes 64 bytes at a time with SIMD. The lexer then
//! writes the whole token stream to flat arrays: the start and the kind of each token. The tokens
//! are the atoms of the class string, the pieces that the parser never splits: a sequence of
//! whitespace, ASCII letters, ASCII digits or `_`, or any other char on its own. Their boundaries
//! come straight from the `atom_start` bitmap.
//!
//! Each atom's kind comes from local rules over the bytes, with no grammar state: a word with a
//! `:` after it outside brackets, before the next whitespace, is a `TW_VARIANT_SEGMENT`; another
//! word outside brackets is a `TW_BASE`; a word inside brackets is a CSS identifier or keyword.
//! The parser never asks for a token to be lexed again. When a token of the tree spans several
//! atoms, such as the value `red-500`, the parser merges them.

mod classify;
#[cfg(test)]
mod tests;

use super::{BaseNameStore, css_identifier_kind};
use crate::syntax::parse_error::invalid_whitespace;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_tailwind_syntax::TextSize;
use classify::{Block, classify};
use std::cell::Cell;

/// The buffers of a [SimdLexer]: its blocks, starts and kinds.
type Buffers = (Vec<Block>, Vec<u32>, Vec<TailwindSyntaxKind>);

/// The largest number of atoms whose buffers [SimdLexer] keeps for the next class string, so
/// that parsing one long class string doesn't keep its memory.
const MAX_KEPT_ATOMS: usize = 1 << 14;

thread_local! {
    /// The buffers of the last [SimdLexer] dropped on this thread, so that lexing a class string
    /// doesn't allocate them.
    ///
    /// This exists purely to reduce pressure on the memory allocator.
    static BUFFERS: Cell<Option<Buffers>> = const { Cell::new(None) };
}

/// The atoms of a class string.
pub(crate) struct SimdLexer<'src> {
    source: &'src str,
    base_names: &'src BaseNameStore,
    blocks: Vec<Block>,
    /// The start of each atom, followed by the source length.
    starts: Vec<u32>,
    /// The kind of each atom, followed by `EOF`.
    kinds: Vec<TailwindSyntaxKind>,
    /// The diagnostics of the whitespace that CSS doesn't allow.
    diagnostics: Vec<ParseDiagnostic>,
}

impl Drop for SimdLexer<'_> {
    fn drop(&mut self) {
        if self.starts.capacity() <= MAX_KEPT_ATOMS {
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

impl<'src> SimdLexer<'src> {
    /// Lexes `source` into atoms. The parser matches dashed basenames against `base_names`.
    pub(crate) fn new(source: &'src str, base_names: &'src BaseNameStore) -> Self {
        let (blocks, starts, kinds) = BUFFERS
            .try_with(|cell| cell.take())
            .ok()
            .flatten()
            .unwrap_or_default();
        let mut lexer = Self {
            source,
            base_names,
            blocks,
            starts,
            kinds,
            diagnostics: Vec::new(),
        };
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("bmi1")
            && std::arch::is_x86_feature_detected!("bmi2")
            && std::arch::is_x86_feature_detected!("lzcnt")
            && std::arch::is_x86_feature_detected!("popcnt")
        {
            // SAFETY: the CPU supports the features.
            unsafe { lexer.lex_with_bit_instructions() };
            return lexer;
        }
        lexer.lex();
        lexer
    }

    /// [Self::lex], compiled with the bit manipulation instructions of x86_64 CPUs since 2013,
    /// which the extraction of atom starts uses for every atom.
    ///
    /// # Safety
    /// The CPU must support AVX2, BMI1, BMI2, LZCNT and POPCNT.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
    unsafe fn lex_with_bit_instructions(&mut self) {
        self.lex();
    }

    #[inline(always)]
    fn lex(&mut self) {
        classify(self.source.as_bytes(), &mut self.blocks);
        self.split_hex_colors();
        self.write_starts();
        self.write_kinds();
    }

    /// Splits a hex color from the ASCII letters after it in the same word, as in `#abcxyz`, so
    /// that the color ends on an atom boundary.
    fn split_hex_colors(&mut self) {
        let bytes = self.source.as_bytes();
        for block_index in 0..self.blocks.len() {
            let mut hashes = self.blocks[block_index].hash;
            while hashes != 0 {
                let hash = block_index * 64 + hashes.trailing_zeros() as usize;
                hashes &= hashes - 1;
                let mut end = hash + 1;
                while bytes.get(end).is_some_and(u8::is_ascii_hexdigit) {
                    end += 1;
                }
                if bytes.get(end).is_some_and(u8::is_ascii_alphanumeric)
                    && bytes[end - 1].is_ascii_alphabetic()
                {
                    self.blocks[end / 64].atom_start |= 1 << (end % 64);
                }
            }
        }
    }

    /// Writes the start of each atom, and the source length after them.
    #[inline(always)]
    fn write_starts(&mut self) {
        let len = self.source.len();
        let starts = &mut self.starts;
        starts.clear();
        // Each atom takes at least one byte.
        starts.reserve(len + 1);
        let out = &mut starts.spare_capacity_mut()[..len + 1];
        let mut count = 0;
        for (block_index, block) in self.blocks.iter().enumerate() {
            let mut bits = block.atom_start;
            while bits != 0 {
                out[count].write((block_index * 64) as u32 + bits.trailing_zeros());
                count += 1;
                bits &= bits - 1;
            }
        }
        out[count].write(len as u32);
        // SAFETY: the loop and the write after it initialized the first `count + 1` starts.
        unsafe { starts.set_len(count + 1) };
    }

    /// Writes the kind of each atom, and `EOF` after them.
    fn write_kinds(&mut self) {
        let bytes = self.source.as_bytes();
        let count = self.starts.len() - 1;
        let kinds = &mut self.kinds;
        kinds.clear();
        kinds.reserve(count + 1);
        // How many brackets and parentheses are open in the current class.
        let mut depth = 0u32;
        // The first atom of the current class that a `:` may still make a variant segment.
        let mut unresolved = 0;
        for index in 0..count {
            let start = self.starts[index] as usize;
            let end = self.starts[index + 1] as usize;
            let byte = bytes[start];
            let kind = match ATOM_KINDS[byte as usize] {
                WHITESPACE => {
                    depth = 0;
                    unresolved = index + 1;
                    if any_in(&self.blocks, start, end, |block| block.bad_whitespace) {
                        push_whitespace_diagnostics(bytes, start, end, &mut self.diagnostics);
                    }
                    WHITESPACE
                }
                TW_BASE if depth > 0 => css_identifier_kind(&bytes[start..end]),
                TW_NUMBER if depth > 0 => CSS_NUMBER_LITERAL,
                kind @ (T!['['] | T!['(']) => {
                    depth += 1;
                    kind
                }
                kind @ (T![']'] | T![')']) => {
                    depth = depth.saturating_sub(1);
                    kind
                }
                T![:] => {
                    if depth == 0 {
                        for kind in &mut kinds[unresolved..index] {
                            if *kind == TW_BASE {
                                *kind = TW_VARIANT_SEGMENT;
                            }
                        }
                        unresolved = index + 1;
                    }
                    T![:]
                }
                ERROR_TOKEN if start == 0 && bytes.starts_with(&[0xEF, 0xBB, 0xBF]) => UNICODE_BOM,
                kind => kind,
            };
            kinds.push(kind);
        }
        kinds.push(EOF);
    }

    pub(crate) fn source(&self) -> &'src str {
        self.source
    }

    /// Returns the kind of atom `index`, or `EOF` past the last atom.
    #[inline(always)]
    pub(crate) fn kind(&self, index: usize) -> TailwindSyntaxKind {
        self.kinds.get(index).copied().unwrap_or(EOF)
    }

    /// Returns the start of atom `index`, or the source length past the last atom.
    #[inline(always)]
    pub(crate) fn start(&self, index: usize) -> usize {
        self.starts
            .get(index)
            .map_or(self.source.len(), |&start| start as usize)
    }

    /// Returns the first byte of atom `index`, or `None` past the last atom.
    #[inline(always)]
    pub(crate) fn byte(&self, index: usize) -> Option<u8> {
        self.source.as_bytes().get(self.start(index)).copied()
    }

    /// Returns the text of atom `index`, which is empty past the last atom.
    #[inline(always)]
    pub(crate) fn text(&self, index: usize) -> &'src str {
        &self.source[self.start(index)..self.start(index + 1)]
    }

    /// Returns the index of the atom that starts at `offset`, searching forward from atom
    /// `from`. The offset must be an atom boundary.
    #[inline(always)]
    pub(crate) fn index_at(&self, offset: usize, from: usize) -> usize {
        let mut index = from;
        while self.start(index) < offset {
            index += 1;
        }
        debug_assert_eq!(self.start(index), offset);
        index
    }

    /// Returns the end of the basename that starts at `start` and runs to `end`, before a `-`,
    /// continued over each `-` that a dashed basename continues over.
    #[inline(always)]
    pub(crate) fn dashed_base_end(&self, start: usize, end: usize) -> usize {
        start
            + self
                .base_names
                .matcher(&self.source.as_bytes()[start..])
                .dashed_end(end - start)
    }

    /// Returns whether a `:` follows `offset` before the next whitespace.
    #[inline(always)]
    pub(crate) fn has_colon_before_whitespace(&self, offset: usize) -> bool {
        let end = next_bit(&self.blocks, offset, |block| block.colon_stop);
        self.source.as_bytes().get(end) == Some(&b':')
    }

    /// Returns whether `start..end` holds a line break.
    #[inline(always)]
    pub(crate) fn has_line_break(&self, start: usize, end: usize) -> bool {
        start < end && any_in(&self.blocks, start, end, |block| block.line_break)
    }

    /// Takes the diagnostics of the whitespace that CSS doesn't allow.
    pub(crate) fn take_diagnostics(&mut self) -> Vec<ParseDiagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

/// The kind of an atom by its first byte, before the rules that look around it: `TW_BASE` for a
/// word, `TW_NUMBER` for digits, and `ERROR_TOKEN` for a char without a kind of its own.
static ATOM_KINDS: [TailwindSyntaxKind; 256] = {
    let mut table = [ERROR_TOKEN; 256];
    let mut byte = 0;
    while byte < 256 {
        table[byte] = match byte as u8 {
            b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ' => WHITESPACE,
            b'a'..=b'z' | b'A'..=b'Z' => TW_BASE,
            b'0'..=b'9' => TW_NUMBER,
            b'_' => CSS_WHITESPACE,
            b'(' => T!['('],
            b')' => T![')'],
            b'[' => T!['['],
            b']' => T![']'],
            b':' => T![:],
            b'-' => T![-],
            b'!' => T![!],
            b'/' => T![/],
            b'%' => T![%],
            b'+' => T![+],
            b'*' => T![*],
            b',' => T![,],
            b'.' => T![.],
            b'=' => T![=],
            b'#' => T![#],
            _ => ERROR_TOKEN,
        };
        byte += 1;
    }
    table
};

/// Pushes a diagnostic for each whitespace byte in `start..end` that CSS doesn't allow.
#[cold]
fn push_whitespace_diagnostics(
    bytes: &[u8],
    start: usize,
    end: usize,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    for (offset, &byte) in bytes[start..end].iter().enumerate() {
        if matches!(byte, 0x0B | 0x0C) {
            diagnostics.push(invalid_whitespace(TextSize::from((start + offset) as u32)));
        }
    }
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

/// Returns whether `field` has a bit in `from..to`, which must not be empty.
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
