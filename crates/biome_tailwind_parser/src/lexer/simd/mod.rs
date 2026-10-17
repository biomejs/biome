//! A lexer that classifies the whole class string with SIMD before parsing starts.
//!
//! [classify] computes bitmaps of character classes 64 bytes at a time, without context. They
//! split the source into atoms, and for each kind of token they mark the atoms that end it. With
//! them, the token source's groupings find the end of most tokens with a search for the next set
//! bit of one bitmap, plus short scalar loops for CSS values, strings and nested brackets.

mod classify;
#[cfg(test)]
mod tests;

use super::{
    BaseNameStore, TailwindLexContext, css_identifier_kind, invalid_whitespace,
    unexpected_character,
};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_tailwind_syntax::TextSize;
use classify::{Block, classify};
use std::cell::Cell;

/// The largest number of blocks whose buffer [SimdLexer] keeps for the next class string, so
/// that parsing one long class string doesn't keep its memory.
const MAX_KEPT_BLOCKS: usize = 1 << 10;

thread_local! {
    /// The bitmap buffer of the last [SimdLexer] dropped on this thread, so that classifying a
    /// class string doesn't allocate it.
    static BUFFERS: Cell<Option<Vec<Block>>> = const { Cell::new(None) };
}

/// The bitmaps of a class string.
pub(crate) struct SimdLexer<'src> {
    source: &'src str,
    base_names: &'src BaseNameStore,
    blocks: Vec<Block>,
}

impl Drop for SimdLexer<'_> {
    fn drop(&mut self) {
        if self.blocks.capacity() <= MAX_KEPT_BLOCKS {
            let blocks = std::mem::take(&mut self.blocks);
            // The thread may be ending, in which case the buffer is dropped.
            let _ = BUFFERS.try_with(|cell| cell.set(Some(blocks)));
        }
    }
}

impl<'src> SimdLexer<'src> {
    /// Classifies `source`, matching dashed basenames against `base_names`.
    pub(crate) fn new(source: &'src str, base_names: &'src BaseNameStore) -> Self {
        let mut blocks = BUFFERS
            .try_with(|cell| cell.take())
            .ok()
            .flatten()
            .unwrap_or_default();
        classify(source.as_bytes(), &mut blocks);
        Self {
            source,
            base_names,
            blocks,
        }
    }

    pub(crate) fn source(&self) -> &'src str {
        self.source
    }

    /// Returns whether a `:` follows `offset` before the next whitespace.
    #[inline(always)]
    pub(crate) fn has_colon_before_whitespace(&self, offset: usize) -> bool {
        let end = next_bit(&self.blocks, offset, |block| block.colon_stop);
        self.source.as_bytes().get(end) == Some(&b':')
    }

    /// Returns the offset of the first atom at or after `position` that `stop` matches, or the
    /// source length if there is none.
    #[inline(always)]
    pub(crate) fn find_stop(&self, position: usize, stop: Stop) -> usize {
        match stop {
            Stop::ParenOrWhitespace => next_bit(&self.blocks, position, |block| block.paren_stop),
            Stop::ColonOrWhitespace => next_bit(&self.blocks, position, |block| block.colon_stop),
            Stop::Segment => next_bit(&self.blocks, position, |block| block.segment_stop),
            Stop::Value => next_bit(&self.blocks, position, |block| block.value_stop),
            Stop::Modifier => next_bit(&self.blocks, position, |block| block.modifier_stop),
        }
    }

    /// Returns whether `start..end` holds a line break or a whitespace byte that CSS doesn't
    /// allow.
    #[inline(always)]
    pub(crate) fn has_line_break_or_bad_whitespace(&self, start: usize, end: usize) -> bool {
        start < end
            && any_in(&self.blocks, start, end, |block| {
                block.line_break | block.bad_whitespace
            })
    }

    /// Returns whether `start..end` holds a line break.
    #[inline(always)]
    pub(crate) fn has_line_break(&self, start: usize, end: usize) -> bool {
        start < end && any_in(&self.blocks, start, end, |block| block.line_break)
    }

    /// Lexes the token at `position` in `context`, after a token of kind `previous`, and pushes
    /// its diagnostics to `diagnostics`. Returns the token's kind and end.
    pub(crate) fn lex(
        &self,
        position: usize,
        context: TailwindLexContext,
        previous: TailwindSyntaxKind,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> (TailwindSyntaxKind, usize) {
        let bytes = self.source.as_bytes();
        let Some(&byte) = bytes.get(position) else {
            return (EOF, position);
        };
        let (kind, end) = if is_whitespace(byte) {
            (WHITESPACE, self.whitespace_end(position))
        } else {
            match context {
                TailwindLexContext::CssValue => {
                    return self.css_value(position, previous, diagnostics);
                }
            }
        };
        if kind == WHITESPACE {
            self.push_whitespace_diagnostics(position, end, diagnostics);
        }
        (kind, end)
    }

    /// Lexes the token at `position` in [TailwindLexContext::CssValue], after a token of kind
    /// `previous`.
    fn css_value(
        &self,
        position: usize,
        previous: TailwindSyntaxKind,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> (TailwindSyntaxKind, usize) {
        let bytes = self.source.as_bytes();
        let byte = bytes[position];
        let next = bytes.get(position + 1).copied();
        match byte {
            b'(' | b')' | b'[' | b']' | b':' => single(byte, position),
            // In math functions, Tailwind adds whitespace around a `-` that directly follows a
            // value, so it is a binary operator rather than the start of a negative number.
            b'-' if position > 0
                && (bytes[position - 1].is_ascii_alphanumeric()
                    || matches!(bytes[position - 1], b'%' | b')')) =>
            {
                single(byte, position)
            }
            b'-' if next.is_some_and(is_css_identifier_start) => self.css_identifier(position),
            b'-' if next.is_some_and(|next| next.is_ascii_digit() || next == b'.') => {
                css_number(bytes, position)
            }
            b'0'..=b'9' => css_number(bytes, position),
            b'.' if next.is_some_and(|next| next.is_ascii_digit()) => css_number(bytes, position),
            b'_' => {
                let end = bytes[position..]
                    .iter()
                    .position(|&byte| byte != b'_')
                    .map_or(bytes.len(), |len| position + len);
                (CSS_WHITESPACE, end)
            }
            b'a'..=b'z' | b'A'..=b'Z' if previous == CSS_DIMENSION_VALUE => {
                let end = bytes[position..]
                    .iter()
                    .position(|&byte| !byte.is_ascii_alphabetic())
                    .map_or(bytes.len(), |len| position + len);
                (css_identifier_kind(&bytes[position..end]), end)
            }
            b'a'..=b'z' | b'A'..=b'Z' => self.css_identifier(position),
            b'"' | b'\'' => (CSS_STRING_LITERAL, css_string_end(bytes, position)),
            b'#' => {
                let end = bytes[position + 1..]
                    .iter()
                    .position(|byte| !byte.is_ascii_hexdigit())
                    .map_or(bytes.len(), |len| position + 1 + len);
                (CSS_COLOR_LITERAL, end)
            }
            b'-' | b'%' | b'+' | b'*' | b',' | b'.' | b'=' | b'/' => single(byte, position),
            _ => self.unexpected(position, diagnostics),
        }
    }

    /// Lexes the CSS identifier at `position`. A custom property, which starts with `--`, also
    /// takes `_` and non-ASCII chars, which Tailwind keeps in its name.
    fn css_identifier(&self, position: usize) -> (TailwindSyntaxKind, usize) {
        let bytes = self.source.as_bytes();
        let is_custom_property = bytes[position..].starts_with(b"--");
        let mut end = position;
        while let Some(&byte) = bytes.get(end) {
            if byte.is_ascii_alphanumeric() || byte == b'-' {
                end += 1;
            } else if is_custom_property && (byte == b'_' || !byte.is_ascii()) {
                end += utf8_len(byte);
            } else {
                break;
            }
        }
        (css_identifier_kind(&bytes[position..end]), end)
    }

    /// Returns an error token for the char at `position`, and pushes its diagnostic.
    #[cold]
    pub(crate) fn unexpected(
        &self,
        position: usize,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> (TailwindSyntaxKind, usize) {
        let char = self.source[position..].chars().next().unwrap_or_default();
        diagnostics.push(unexpected_character(char, TextSize::from(position as u32)));
        (ERROR_TOKEN, position + char.len_utf8())
    }

    /// Pushes a diagnostic for each whitespace byte in `start..end` that CSS doesn't allow.
    pub(crate) fn push_whitespace_diagnostics(
        &self,
        start: usize,
        end: usize,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) {
        if !any_in(&self.blocks, start, end, |block| block.bad_whitespace) {
            return;
        }
        for (offset, &byte) in self.source.as_bytes()[start..end].iter().enumerate() {
            if matches!(byte, 0x0B | 0x0C) {
                diagnostics.push(invalid_whitespace(TextSize::from((start + offset) as u32)));
            }
        }
    }

    /// Returns the end of the whitespace at `position`.
    #[inline(always)]
    pub(crate) fn whitespace_end(&self, position: usize) -> usize {
        next_bit(&self.blocks, position + 1, |block| !block.whitespace)
    }

    /// Returns the end of the selector at `position`, counting nested brackets.
    #[inline(always)]
    pub(crate) fn selector_end(&self, position: usize) -> usize {
        let end = next_bit(&self.blocks, position, |block| block.selector_stop);
        let bytes = self.source.as_bytes();
        if end < bytes.len() && bytes[end] == b'[' {
            nested_selector_end(bytes, end)
        } else {
            end
        }
    }

    /// Returns `TW_NUMBER` if `start..end` holds only digits and `.`, and `TW_VALUE` otherwise.
    #[inline(always)]
    pub(crate) fn number_or_value(&self, start: usize, end: usize) -> TailwindSyntaxKind {
        if next_bit(&self.blocks, start, |block| !block.numeric) >= end {
            TW_NUMBER
        } else {
            TW_VALUE
        }
    }

    /// Lexes the basename at `start`, matching dashed basenames.
    #[inline(always)]
    pub(crate) fn base(&self, start: usize) -> (TailwindSyntaxKind, usize) {
        let source = self.source.as_bytes();
        let end = next_bit(&self.blocks, start + 1, |block| block.base_stop);
        if &source[start..end] == b"data" {
            return (DATA_KW, end);
        }
        if source.get(end) != Some(&b'-') {
            return (TW_BASE, end);
        }
        // A dashed basename continues past a '-', so it isn't `data`.
        let end = start
            + self
                .base_names
                .matcher(&source[start..])
                .dashed_end(end - start);
        (TW_BASE, end)
    }
}

/// The atoms that end a token that [SimdLexer::find_stop] looks for.
///
/// The lexer splits the source into atoms: sequences of whitespace, ASCII letters, ASCII digits
/// or `_`, and single bytes or chars of any other kind. Each stop is a set of punctuation bytes and
/// whitespace, so the first atom that it matches starts at the first byte in its bitmap.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Stop {
    /// `)` and whitespace.
    ParenOrWhitespace,
    /// `:` and whitespace.
    ColonOrWhitespace,
    /// The end of a variant segment name: whitespace, `:`, `-`, `/`, `!`, `[`, `]` and `)`.
    Segment,
    /// The end of a value after `-`: whitespace, `:`, `/`, `!`, `]`, `)` and `%`.
    Value,
    /// The end of a modifier after `/`: whitespace, `!`, `%` and `:`.
    Modifier,
}

/// Returns the kind and end of the one-byte token `byte` at `position`.
#[inline(always)]
fn single(byte: u8, position: usize) -> (TailwindSyntaxKind, usize) {
    (punctuation(byte), position + 1)
}

/// Returns the kind of the one-byte token `byte`, which must be one of `()[]:-!/%+*,.=`.
#[inline(always)]
pub(crate) fn punctuation(byte: u8) -> TailwindSyntaxKind {
    match byte {
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
        _ => unreachable!("not a one-byte token: {byte:?}"),
    }
}

/// Whether the lexer reads `byte` as whitespace.
#[inline(always)]
pub(crate) fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ')
}

/// Whether `byte` can start a CSS identifier after `-`: an ASCII letter or `-`.
#[inline(always)]
fn is_css_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'-'
}

/// Returns the end of the CSS number at `start`, and its kind: a percentage if `%` follows, a
/// dimension if a unit follows, and a number otherwise.
fn css_number(bytes: &[u8], start: usize) -> (TailwindSyntaxKind, usize) {
    let digits = |from: usize| {
        bytes[from..]
            .iter()
            .position(|byte| !byte.is_ascii_digit())
            .map_or(bytes.len(), |len| from + len)
    };
    let mut end = start;
    if matches!(bytes.get(end), Some(b'+' | b'-')) {
        end += 1;
    }
    end = digits(end);
    if bytes.get(end) == Some(&b'.') && bytes.get(end + 1).is_some_and(u8::is_ascii_digit) {
        end = digits(end + 1);
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let mut exponent = end + 1;
        if matches!(bytes.get(exponent), Some(b'+' | b'-')) {
            exponent += 1;
        }
        if bytes.get(exponent).is_some_and(u8::is_ascii_digit) {
            end = digits(exponent);
        }
    }
    let kind = match bytes.get(end) {
        Some(b'%') => CSS_PERCENTAGE_VALUE,
        Some(byte) if byte.is_ascii_alphabetic() => CSS_DIMENSION_VALUE,
        _ => CSS_NUMBER_LITERAL,
    };
    (kind, end)
}

/// Returns the end of the CSS string at `start`, which runs to its closing quote or to the next
/// whitespace.
pub(crate) fn css_string_end(bytes: &[u8], start: usize) -> usize {
    let quote = bytes[start];
    let mut end = start + 1;
    while let Some(&byte) = bytes.get(end) {
        if is_whitespace(byte) {
            break;
        }
        end += 1;
        if byte == quote {
            break;
        }
        if byte == b'\\'
            && bytes
                .get(end)
                .is_some_and(|&escaped| !is_whitespace(escaped))
        {
            end += 1;
        }
    }
    end
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
            b']' => break,
            _ if is_whitespace(byte) => break,
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
