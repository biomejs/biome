//! Character classes of the source bytes, computed 64 bytes at a time.
//!
//! Each class is a bitmap per 64-byte block, where bit `i` describes byte `i` of the block.
//! x86_64 uses AVX2 when the CPU has it and SSE2 otherwise; other targets classify one byte at a
//! time with a lookup table.
//!
//! The `*_stop` classes are the bytes that end a token of some kind, and they include every
//! position past the end of the source, so that searching one of them for the end of a token
//! always finds one.

/// The class bitmaps of one 64-byte block.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Block {
    /// The bytes the lexer reads as whitespace.
    pub(super) whitespace: u64,
    /// The whitespace bytes the lexer reports a diagnostic for: `\x0B` and `\x0C`.
    pub(super) bad_whitespace: u64,
    /// `\n` and `\r`.
    pub(super) line_break: u64,
    /// The bytes that end a basename that the dashed basenames don't extend: `-`, whitespace,
    /// `!`, `:` and `/`.
    pub(super) base_stop: u64,
    /// The bytes that end a value after `-`: whitespace, `:`, `/`, `!`, `]`, `)` and `%`.
    pub(super) value_stop: u64,
    /// The bytes that end a modifier after `/`: whitespace, `!`, `%` and `:`.
    pub(super) modifier_stop: u64,
    /// The bytes that end a selector after `[`, unless it nests brackets: whitespace, `[` and
    /// `]`.
    pub(super) selector_stop: u64,
    /// The bytes that end a value after `(`: whitespace and `)`.
    pub(super) paren_stop: u64,
    /// The bytes that end a variant segment name: whitespace, `:`, `-`, `/`, `!`, `[`, `]` and
    /// `)`.
    pub(super) segment_stop: u64,
    /// ASCII digits and `.`.
    pub(super) numeric: u64,
    /// The starts of the tokens in [TailwindLexContext::VariantSegment], which depend on the
    /// offset alone, plus every position past the end of the source.
    ///
    /// [TailwindLexContext::VariantSegment]: crate::token_source::TailwindLexContext::VariantSegment
    pub(super) segment_starts: u64,
}

const WHITESPACE: u16 = 1 << 0;
const BAD_WHITESPACE: u16 = 1 << 1;
const LINE_BREAK: u16 = 1 << 2;
const BASE_STOP: u16 = 1 << 3;
const VALUE_STOP: u16 = 1 << 4;
const MODIFIER_STOP: u16 = 1 << 5;
const SELECTOR_STOP: u16 = 1 << 6;
const PAREN_STOP: u16 = 1 << 7;
const SEGMENT_STOP: u16 = 1 << 8;
const NUMERIC: u16 = 1 << 9;
/// The bytes that start a variant segment name: ASCII letters and digits, `_`, `@` and `*`.
const NAME_START: u16 = 1 << 10;

/// The classes of an ASCII byte.
const fn ascii_classes(byte: u8) -> u16 {
    let whitespace = matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ');
    let mut classes = 0;
    if whitespace {
        classes |= WHITESPACE;
    }
    if matches!(byte, 0x0B | 0x0C) {
        classes |= BAD_WHITESPACE;
    }
    if matches!(byte, b'\n' | b'\r') {
        classes |= LINE_BREAK;
    }
    if whitespace || matches!(byte, b'-' | b'!' | b':' | b'/') {
        classes |= BASE_STOP;
    }
    if whitespace || matches!(byte, b':' | b'/' | b'!' | b']' | b')' | b'%') {
        classes |= VALUE_STOP;
    }
    if whitespace || matches!(byte, b'!' | b'%' | b':') {
        classes |= MODIFIER_STOP;
    }
    if whitespace || matches!(byte, b'[' | b']') {
        classes |= SELECTOR_STOP;
    }
    if whitespace || byte == b')' {
        classes |= PAREN_STOP;
    }
    if whitespace || matches!(byte, b':' | b'-' | b'/' | b'!' | b'[' | b']' | b')') {
        classes |= SEGMENT_STOP;
    }
    if byte.is_ascii_digit() || byte == b'.' {
        classes |= NUMERIC;
    }
    if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'@' | b'*') {
        classes |= NAME_START;
    }
    classes
}

static ASCII_CLASSES: [u16; 128] = {
    let mut table = [0; 128];
    let mut byte = 0;
    while byte < 128 {
        table[byte] = ascii_classes(byte as u8);
        byte += 1;
    }
    table
};

/// The classes of the 64 bytes of a block, before the variant segment starts are derived.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Classes {
    whitespace: u64,
    bad_whitespace: u64,
    line_break: u64,
    base_stop: u64,
    value_stop: u64,
    modifier_stop: u64,
    selector_stop: u64,
    paren_stop: u64,
    segment_stop: u64,
    numeric: u64,
    name_start: u64,
    non_ascii: u64,
    /// UTF-8 continuation bytes, `0x80` to `0xBF`.
    continuation: u64,
}

#[cfg(any(test, not(target_arch = "x86_64")))]
fn classify_scalar(bytes: &[u8; 64], len: usize) -> Classes {
    let mut classes = Classes::default();
    for (index, &byte) in bytes[..len].iter().enumerate() {
        let bit = 1u64 << index;
        if byte >= 0x80 {
            classes.non_ascii |= bit;
            if byte < 0xC0 {
                classes.continuation |= bit;
            }
            continue;
        }
        let byte_classes = ASCII_CLASSES[byte as usize];
        let set = |class: u16| if byte_classes & class != 0 { bit } else { 0 };
        classes.whitespace |= set(WHITESPACE);
        classes.bad_whitespace |= set(BAD_WHITESPACE);
        classes.line_break |= set(LINE_BREAK);
        classes.base_stop |= set(BASE_STOP);
        classes.value_stop |= set(VALUE_STOP);
        classes.modifier_stop |= set(MODIFIER_STOP);
        classes.selector_stop |= set(SELECTOR_STOP);
        classes.paren_stop |= set(PAREN_STOP);
        classes.segment_stop |= set(SEGMENT_STOP);
        classes.numeric |= set(NUMERIC);
        classes.name_start |= set(NAME_START);
    }
    classes
}

/// Combines the classes of consecutive blocks into [Block]s, carrying the state that the
/// variant segment starts depend on across block boundaries.
#[derive(Default)]
struct SegmentTracker {
    /// Whether the last byte of the previous block is whitespace.
    previous_whitespace: u64,
    /// Whether the last byte of the previous block is in a name or a prefix, defined below.
    previous_inside: u64,
    /// Whether the last byte of the previous block is in a name.
    previous_name: u64,
    /// The carry out of the addition that finds the prefixes in the previous block.
    prefix_carry: u64,
}

impl SegmentTracker {
    /// Returns the block of `classes`, where `valid` marks the bytes of the source.
    ///
    /// In [TailwindLexContext::VariantSegment], the bytes between two `segment_stop` bytes form
    /// a name from the first byte that starts one. The bytes before it, the prefix, are each a
    /// token of their own: `(` or a char the lexer rejects. Every `segment_stop` byte other than
    /// whitespace is a token of its own, and consecutive whitespace is one token.
    ///
    /// [TailwindLexContext::VariantSegment]: crate::token_source::TailwindLexContext::VariantSegment
    #[inline(always)]
    fn block(&mut self, classes: Classes, valid: u64) -> Block {
        let invalid = !valid;
        let whitespace = classes.whitespace & valid;
        let segment_stop = classes.segment_stop & valid;
        let inside = valid & !segment_stop;
        let seeds = classes.name_start & inside;

        // A prefix is a sequence of non-seed bytes at the start of a sequence of `inside` bytes.
        // Adding its first bit to `plain` carries through it and clears it.
        let plain = inside & !seeds;
        let inside_starts = inside & !((inside << 1) | self.previous_inside);
        let prefix_starts = inside_starts & plain;
        let (sum, carry_low) = plain.overflowing_add(prefix_starts);
        let (sum, carry_high) = sum.overflowing_add(self.prefix_carry);
        let prefix = (sum ^ plain) & plain;
        let name = inside & !prefix;

        let whitespace_starts = whitespace & !((whitespace << 1) | self.previous_whitespace);
        let name_starts = name & !((name << 1) | self.previous_name);
        let prefix_chars = prefix & !classes.continuation;
        let segment_starts =
            whitespace_starts | (segment_stop & !whitespace) | name_starts | prefix_chars | invalid;

        self.previous_whitespace = whitespace >> 63;
        self.previous_inside = inside >> 63;
        self.previous_name = name >> 63;
        self.prefix_carry = u64::from(carry_low | carry_high);

        Block {
            whitespace,
            bad_whitespace: classes.bad_whitespace & valid,
            line_break: classes.line_break & valid,
            base_stop: classes.base_stop | invalid,
            value_stop: classes.value_stop | invalid,
            modifier_stop: classes.modifier_stop | invalid,
            selector_stop: classes.selector_stop | invalid,
            paren_stop: classes.paren_stop | invalid,
            segment_stop: classes.segment_stop | invalid,
            numeric: classes.numeric & valid,
            segment_starts,
        }
    }
}

/// Classifies `source` into `blocks`, one [Block] per 64 bytes plus one trailing block past the
/// end of the source.
pub(super) fn classify(source: &[u8], blocks: &mut Vec<Block>) {
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: the CPU supports AVX2.
            return unsafe { x86::classify_avx2(source, blocks) };
        }
        // SAFETY: SSE2 is part of the x86_64 baseline.
        unsafe { x86::classify_sse2(source, blocks) }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        classify_with(source, blocks, classify_scalar)
    }
}

/// Classifies `source` into `blocks` with `classify_block`, which receives each block's 64
/// bytes and how many of them are in the source; the bytes after those are zeros, which have no
/// class.
#[inline(always)]
fn classify_with(
    source: &[u8],
    blocks: &mut Vec<Block>,
    mut classify_block: impl FnMut(&[u8; 64], usize) -> Classes,
) {
    blocks.clear();
    blocks.reserve(source.len() / 64 + 2);
    let mut tracker = SegmentTracker::default();
    let (chunks, tail) = source.as_chunks::<64>();
    for bytes in chunks {
        blocks.push(tracker.block(classify_block(bytes, 64), u64::MAX));
    }
    if !tail.is_empty() {
        let mut bytes = [0; 64];
        bytes[..tail.len()].copy_from_slice(tail);
        let valid = (1u64 << tail.len()) - 1;
        blocks.push(tracker.block(classify_block(&bytes, tail.len()), valid));
    }
    blocks.push(SENTINEL);
}

/// The block after the last byte of the source.
const SENTINEL: Block = Block {
    whitespace: 0,
    bad_whitespace: 0,
    line_break: 0,
    base_stop: u64::MAX,
    value_stop: u64::MAX,
    modifier_stop: u64::MAX,
    selector_stop: u64::MAX,
    paren_stop: u64::MAX,
    segment_stop: u64::MAX,
    numeric: 0,
    segment_starts: u64::MAX,
};

#[cfg(target_arch = "x86_64")]
mod x86 {
    use super::{
        ASCII_CLASSES, BAD_WHITESPACE, BASE_STOP, Block, Classes, LINE_BREAK, MODIFIER_STOP,
        NAME_START, NUMERIC, PAREN_STOP, SEGMENT_STOP, SELECTOR_STOP, VALUE_STOP, WHITESPACE,
        classify_with,
    };
    use std::arch::x86_64::*;

    /// For each class, a bitset of the ASCII bytes in it: bit `byte & 7` of entry `byte >> 3`.
    const fn bitset(class: u16) -> [u8; 16] {
        let mut table = [0; 16];
        let mut byte = 0;
        while byte < 128 {
            if ASCII_CLASSES[byte] & class != 0 {
                table[byte >> 3] |= 1 << (byte & 7);
            }
            byte += 1;
        }
        table
    }

    static BITSETS: [[u8; 16]; 11] = [
        bitset(WHITESPACE),
        bitset(BAD_WHITESPACE),
        bitset(LINE_BREAK),
        bitset(BASE_STOP),
        bitset(VALUE_STOP),
        bitset(MODIFIER_STOP),
        bitset(SELECTOR_STOP),
        bitset(PAREN_STOP),
        bitset(SEGMENT_STOP),
        bitset(NUMERIC),
        bitset(NAME_START),
    ];

    static BITS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

    /// # Safety
    /// The CPU must support AVX2.
    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn classify_avx2(source: &[u8], blocks: &mut Vec<Block>) {
        // SAFETY: the tables are 16 bytes long.
        let load = |table: &[u8; 16]| unsafe {
            _mm256_broadcastsi128_si256(_mm_loadu_si128(table.as_ptr().cast()))
        };
        let bitsets = [
            load(&BITSETS[0]),
            load(&BITSETS[1]),
            load(&BITSETS[2]),
            load(&BITSETS[3]),
            load(&BITSETS[4]),
            load(&BITSETS[5]),
            load(&BITSETS[6]),
            load(&BITSETS[7]),
            load(&BITSETS[8]),
            load(&BITSETS[9]),
            load(&BITSETS[10]),
        ];
        let bits = load(&BITS);
        let low_nibble = _mm256_set1_epi8(0x0F);
        let seven = _mm256_set1_epi8(7);
        let continuation_end = _mm256_set1_epi8(0xC0u8 as i8);

        // Looks up the class bitsets of 32 bytes. Non-ASCII bytes alias ASCII entries, so they
        // are cleared.
        let half = |bytes: &[u8]| {
            // SAFETY: `bytes` is 32 bytes long.
            let v = unsafe { _mm256_loadu_si256(bytes.as_ptr().cast()) };
            let index = _mm256_and_si256(_mm256_srli_epi16::<3>(v), low_nibble);
            let bit = _mm256_shuffle_epi8(bits, _mm256_and_si256(v, seven));
            let non_ascii = _mm256_movemask_epi8(v) as u32 as u64;
            let ascii = !non_ascii;
            let class = |table: __m256i| {
                let hit = _mm256_and_si256(_mm256_shuffle_epi8(table, index), bit);
                (_mm256_movemask_epi8(_mm256_cmpeq_epi8(hit, bit)) as u32 as u64) & ascii
            };
            let continuation = _mm256_movemask_epi8(_mm256_cmpgt_epi8(continuation_end, v)) as u32
                as u64
                & non_ascii;
            [
                class(bitsets[0]),
                class(bitsets[1]),
                class(bitsets[2]),
                class(bitsets[3]),
                class(bitsets[4]),
                class(bitsets[5]),
                class(bitsets[6]),
                class(bitsets[7]),
                class(bitsets[8]),
                class(bitsets[9]),
                class(bitsets[10]),
                non_ascii,
                continuation,
            ]
        };

        classify_with(source, blocks, |bytes, len| {
            let low = half(&bytes[..32]);
            let high = if len > 32 {
                half(&bytes[32..])
            } else {
                [0; 13]
            };
            let join = |index: usize| low[index] | (high[index] << 32);
            Classes {
                whitespace: join(0),
                bad_whitespace: join(1),
                line_break: join(2),
                base_stop: join(3),
                value_stop: join(4),
                modifier_stop: join(5),
                selector_stop: join(6),
                paren_stop: join(7),
                segment_stop: join(8),
                numeric: join(9),
                name_start: join(10),
                non_ascii: join(11),
                continuation: join(12),
            }
        })
    }

    /// # Safety
    /// The CPU must support SSE2, which every x86_64 CPU does.
    #[target_feature(enable = "sse2")]
    pub(super) unsafe fn classify_sse2(source: &[u8], blocks: &mut Vec<Block>) {
        // Classifies 16 bytes with comparisons, as SSE2 has no byte shuffle.
        let quarter = |bytes: &[u8]| {
            // SAFETY: `bytes` is 16 bytes long.
            let v = unsafe { _mm_loadu_si128(bytes.as_ptr().cast()) };
            let eq = |byte: u8| _mm_cmpeq_epi8(v, _mm_set1_epi8(byte as i8));
            let any = |set: &[u8]| {
                set.iter().fold(_mm_setzero_si128(), |acc, &byte| {
                    _mm_or_si128(acc, eq(byte))
                })
            };
            // Whether each byte is in `first..first + len`, as a signed comparison after moving
            // `first` to `i8::MIN`.
            let range = |first: u8, len: u8| {
                let shifted = _mm_add_epi8(v, _mm_set1_epi8(0x80u8.wrapping_sub(first) as i8));
                _mm_cmplt_epi8(shifted, _mm_set1_epi8(0x80u8.wrapping_add(len) as i8))
            };
            let mask = |v: __m128i| _mm_movemask_epi8(v) as u32 as u64;
            let or = _mm_or_si128;

            let whitespace = or(range(b'\t', 5), eq(b' '));
            let digit = range(b'0', 10);
            let alphanumeric = or(digit, or(range(b'a', 26), range(b'A', 26)));
            let non_ascii = mask(v);
            Classes {
                whitespace: mask(whitespace),
                bad_whitespace: mask(range(0x0B, 2)),
                line_break: mask(or(eq(b'\n'), eq(b'\r'))),
                base_stop: mask(or(whitespace, any(b"-!:/"))),
                value_stop: mask(or(whitespace, any(b":/!])%"))),
                modifier_stop: mask(or(whitespace, any(b"!%:"))),
                selector_stop: mask(or(whitespace, any(b"[]"))),
                paren_stop: mask(or(whitespace, eq(b')'))),
                segment_stop: mask(or(whitespace, any(b":-/![])"))),
                numeric: mask(or(digit, eq(b'.'))),
                name_start: mask(or(alphanumeric, any(b"_@*"))),
                non_ascii,
                continuation: mask(_mm_cmplt_epi8(v, _mm_set1_epi8(0xC0u8 as i8))) & non_ascii,
            }
        };

        classify_with(source, blocks, |bytes, len| {
            let part = |index: usize| {
                if len > 16 * index {
                    quarter(&bytes[16 * index..16 * (index + 1)])
                } else {
                    Classes::default()
                }
            };
            let parts = [part(0), part(1), part(2), part(3)];
            let join = |field: fn(&Classes) -> u64| {
                parts
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (index, part)| acc | (field(part) << (16 * index)))
            };
            Classes {
                whitespace: join(|c| c.whitespace),
                bad_whitespace: join(|c| c.bad_whitespace),
                line_break: join(|c| c.line_break),
                base_stop: join(|c| c.base_stop),
                value_stop: join(|c| c.value_stop),
                modifier_stop: join(|c| c.modifier_stop),
                selector_stop: join(|c| c.selector_stop),
                paren_stop: join(|c| c.paren_stop),
                segment_stop: join(|c| c.segment_stop),
                numeric: join(|c| c.numeric),
                name_start: join(|c| c.name_start),
                non_ascii: join(|c| c.non_ascii),
                continuation: join(|c| c.continuation),
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::super::{classify_scalar, classify_with};

        fn sample() -> Vec<u8> {
            let mut bytes: Vec<u8> = (0..=255u8).collect();
            bytes.extend_from_slice(
                "hover:bg-red-500/50 [&_p]:m-[calc(1px+2%)] \u{e9}(x)%y @sm:*:flex\r\n\x0B"
                    .as_bytes(),
            );
            bytes.extend((0..300u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 7) as u8));
            bytes
        }

        #[test]
        fn simd_classes_match_scalar() {
            let source = sample();
            for len in 0..source.len() {
                let source = &source[..len];
                let mut expected = Vec::new();
                classify_with(source, &mut expected, classify_scalar);
                let mut actual = Vec::new();
                // SAFETY: SSE2 is part of the x86_64 baseline.
                unsafe { super::classify_sse2(source, &mut actual) };
                assert_eq!(actual, expected, "sse2, len {len}");
                if std::arch::is_x86_feature_detected!("avx2") {
                    // SAFETY: the CPU supports AVX2.
                    unsafe { super::classify_avx2(source, &mut actual) };
                    assert_eq!(actual, expected, "avx2, len {len}");
                }
            }
        }
    }
}
