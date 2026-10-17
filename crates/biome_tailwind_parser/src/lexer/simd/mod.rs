//! A lexer that classifies the whole class string with SIMD before parsing starts.
//!
//! [classify] computes bitmaps of character classes 64 bytes at a time: for each kind of token,
//! the bytes that end it. With them, lexing any token is a lookup of its first byte and a search
//! for the next set bit of one bitmap, plus short scalar loops for CSS values and nested
//! brackets.
//!
//! Most tokens are lexed in [TailwindLexContext::Regular], where a token depends on its start and
//! on the kind of the token before it, which picks how a token that starts with a name char
//! ends: a basename, a value after `-`, a modifier after `/`, a selector after `[` or a value
//! after `(`. [SimdLexer::new] lexes the whole source in this context, each token after the one
//! before it, into flat arrays of starts and kinds. The token source reads the parser's tokens
//! from these arrays while the parser follows them, and lexes any other token with
//! [SimdLexer::lex].

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

/// Mask of the [TailwindSyntaxKind] in a token's kind and flags.
const KIND_MASK: u16 = 0x00FF;
/// The lexer reports a diagnostic for the token, so [SimdLexer::lex] must lex it again to report
/// it.
pub(crate) const SLOW: u16 = 1 << 8;
/// The token is whitespace with a line break.
pub(crate) const LINE_BREAK: u16 = 1 << 9;
/// The shift of the [previous] class that a regular token was lexed after, if it depends on it.
const PREVIOUS_SHIFT: u16 = 12;

const _: () = assert!(TailwindSyntaxKind::__LAST as u16 <= KIND_MASK);

/// The classes of token kinds that lead to different tokens after them in
/// [TailwindLexContext::Regular].
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

/// The [previous] class of each [TailwindSyntaxKind].
static PREVIOUS_CLASSES: [u16; KIND_MASK as usize + 1] = {
    let mut table = [previous::OTHER; KIND_MASK as usize + 1];
    table[T!['['] as usize] = previous::L_BRACK;
    table[T!['('] as usize] = previous::L_PAREN;
    table[T![-] as usize] = previous::MINUS;
    table[T![/] as usize] = previous::SLASH;
    table
};

/// Returns the [previous] class of `kind`.
#[inline(always)]
pub(crate) fn previous_class(kind: TailwindSyntaxKind) -> u16 {
    PREVIOUS_CLASSES[kind as usize]
}

/// Returns whether the regular token `raw` is the token that [TailwindLexContext::Regular] gives
/// after a token of [previous] class `previous` without a diagnostic.
#[inline(always)]
pub(crate) fn fits_after(raw: u16, previous: u16) -> bool {
    let lexed_after = raw >> PREVIOUS_SHIFT;
    raw & SLOW == 0 && (lexed_after == previous::ANY || lexed_after == previous)
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

/// The blocks, starts and kinds of a [SimdLexer].
type Buffers = (Vec<Block>, Vec<u32>, Vec<u16>);

/// The largest number of tokens whose buffers [SimdLexer] keeps for the next class string, so
/// that parsing one long class string doesn't keep its memory.
const MAX_KEPT_TOKENS: usize = 1 << 14;

thread_local! {
    /// The buffers of the last [SimdLexer] dropped on this thread, so that lexing a class string
    /// doesn't allocate them.
    static BUFFERS: Cell<Option<Buffers>> = const { Cell::new(None) };
}

/// The bitmaps and regular tokens of a class string.
pub(crate) struct SimdLexer<'src> {
    source: &'src str,
    base_names: &'src BaseNameStore,
    blocks: Vec<Block>,
    /// The start of each regular token, followed by the source length.
    starts: Vec<u32>,
    /// The kind and flags of each regular token. The last token is `EOF`.
    kinds: Vec<u16>,
}

impl Drop for SimdLexer<'_> {
    fn drop(&mut self) {
        if self.starts.capacity() <= MAX_KEPT_TOKENS {
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
    /// Classifies `source` and lexes its regular tokens, matching dashed basenames against
    /// `base_names`.
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
        lexer.lex_portable();
        lexer
    }

    /// [Self::lex_portable], compiled with the bit manipulation instructions of x86_64 CPUs
    /// since 2013, which the walk over the bitmaps uses for every token.
    ///
    /// # Safety
    /// The CPU must support AVX2, BMI1, BMI2, LZCNT and POPCNT.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
    unsafe fn lex_with_bit_instructions(&mut self) {
        self.lex_portable();
    }

    #[inline(always)]
    fn lex_portable(&mut self) {
        classify(self.source.as_bytes(), &mut self.blocks);
        self.lex_regular();
    }

    pub(crate) fn source(&self) -> &'src str {
        self.source
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
        }
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
                TailwindLexContext::Regular => {
                    let (raw, end) = self.regular_token(position, previous_class(previous));
                    if raw & SLOW != 0 {
                        return self.unexpected(position, diagnostics);
                    }
                    (kind(raw), end)
                }
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
    fn number_or_value(&self, start: usize, end: usize) -> u16 {
        if next_bit(&self.blocks, start, |block| !block.numeric) >= end {
            TW_NUMBER as u16
        } else {
            TW_VALUE as u16
        }
    }

    /// Lexes the basename at `start`, matching dashed basenames.
    #[inline(always)]
    pub(crate) fn base(&self, start: usize) -> (u16, usize) {
        let source = self.source.as_bytes();
        let end = next_bit(&self.blocks, start + 1, |block| block.base_stop);
        if &source[start..end] == b"data" {
            return (DATA_KW as u16, end);
        }
        if source.get(end) != Some(&b'-') {
            return (TW_BASE as u16, end);
        }
        // A dashed basename continues past a '-', so it isn't `data`.
        let end = start
            + self
                .base_names
                .matcher(&source[start..])
                .dashed_end(end - start);
        (TW_BASE as u16, end)
    }

    /// Lexes the token at `position` in [TailwindLexContext::Regular], after a token of
    /// [previous] class `previous`. Returns its kind and flags, and its end.
    #[inline(always)]
    fn regular_token(&self, position: usize, previous: u16) -> (u16, usize) {
        let source = self.source.as_bytes();
        let byte = source[position];
        let first = REGULAR_FIRST[byte as usize];
        if first == WHITESPACE_FIRST {
            let end = self.whitespace_end(position);
            let flags = if !any_in(&self.blocks, position, end, |block| {
                block.bad_whitespace | block.line_break
            }) {
                0
            } else if any_in(&self.blocks, position, end, |block| block.bad_whitespace) {
                SLOW
            } else {
                LINE_BREAK
            };
            return (WHITESPACE as u16 | flags, end);
        }
        if first & ANY_PREVIOUS_FIRST != 0 {
            return (first & KIND_MASK, position + 1);
        }
        let (kind, end) = match previous {
            previous::L_BRACK => (TW_SELECTOR as u16, self.selector_end(position)),
            previous::L_PAREN => (
                TW_VALUE as u16,
                next_bit(&self.blocks, position + 1, |block| block.paren_stop),
            ),
            previous::MINUS => {
                let end = next_bit(&self.blocks, position, |block| block.value_stop);
                (self.number_or_value(position, end), end)
            }
            previous::SLASH => {
                let end = next_bit(&self.blocks, position, |block| block.modifier_stop);
                let kind = if end == position {
                    ERROR_TOKEN as u16
                } else {
                    self.number_or_value(position, end)
                };
                (kind, end)
            }
            _ => match first {
                BASE_FIRST => self.base(position),
                SEGMENT_FIRST => (
                    TW_VARIANT_SEGMENT as u16,
                    next_bit(&self.blocks, position + 1, |block| block.segment_stop),
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
    }

    /// Lexes the regular tokens of the source, each after the one before it.
    #[inline(always)]
    fn lex_regular(&mut self) {
        let len = self.source.len();
        // Every token takes at least one byte, except for an empty value or modifier, which
        // follows a `-` or `/` of one byte, and `EOF`.
        let capacity = 2 * len + 1;
        let mut starts = std::mem::take(&mut self.starts);
        let mut kinds = std::mem::take(&mut self.kinds);
        starts.clear();
        kinds.clear();
        starts.reserve(capacity + 1);
        kinds.reserve(capacity);
        let starts_out = &mut starts.spare_capacity_mut()[..capacity + 1];
        let kinds_out = &mut kinds.spare_capacity_mut()[..capacity];
        let mut count = 0;
        let mut position = 0;
        // The lexer starts with `EOF` as its current kind.
        let mut previous = previous::OTHER;
        while position < len {
            let (raw, end) = self.regular_token(position, previous);
            starts_out[count].write(position as u32);
            kinds_out[count].write(raw);
            count += 1;
            previous = PREVIOUS_CLASSES[(raw & KIND_MASK) as usize];
            position = end;
        }
        starts_out[count].write(len as u32);
        kinds_out[count].write(EOF as u16);
        starts_out[count + 1].write(len as u32);
        // SAFETY: the loop and the writes after it initialized the first `count + 1` kinds and
        // `count + 2` starts.
        unsafe {
            kinds.set_len(count + 1);
            starts.set_len(count + 2);
        }
        self.starts = starts;
        self.kinds = kinds;
    }

    #[cfg(test)]
    pub(crate) fn regular_tokens(&self) -> impl Iterator<Item = (u32, u32, u16)> + '_ {
        self.kinds
            .iter()
            .enumerate()
            .map(|(index, &kind)| (self.starts[index], self.starts[index + 1], kind))
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
            b'(' => ANY_PREVIOUS_FIRST | T!['('] as u16,
            b')' => ANY_PREVIOUS_FIRST | T![')'] as u16,
            b'[' => ANY_PREVIOUS_FIRST | T!['['] as u16,
            b']' => ANY_PREVIOUS_FIRST | T![']'] as u16,
            b':' => T![:] as u16,
            b'-' => T![-] as u16,
            b'%' => T![%] as u16,
            b'!' => T![!] as u16,
            b'/' => T![/] as u16,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'@' => BASE_FIRST,
            b'*' => SEGMENT_FIRST,
            _ => ERROR_FIRST,
        };
        byte += 1;
    }
    table
};
