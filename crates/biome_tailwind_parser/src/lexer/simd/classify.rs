//! Character classes of the source bytes, computed 64 bytes at a time.
//!
//! Each class is a bitmap per 64-byte block, where bit `i` describes byte `i` of the block.
//! x86_64 uses AVX2 when the CPU has it and SSE2 otherwise; other targets classify one byte at a
//! time with a lookup table.

/// The class bitmaps of one 64-byte block.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Block {
    /// The bytes the lexer reads as whitespace.
    pub(super) whitespace: u64,
    /// The whitespace bytes the lexer reports a diagnostic for: `\x0B` and `\x0C`.
    pub(super) bad_whitespace: u64,
    /// `\n` and `\r`.
    pub(super) line_break: u64,
    /// `:` and whitespace, which bound the variants of a class, and every position past the end
    /// of the source, so that a search for the next one always finds one.
    pub(super) colon_stop: u64,
    /// `#`, after which the lexer splits a hex color from the letters after it.
    pub(super) hash: u64,
    /// The first byte of each letter: of each sequence of whitespace, ASCII letters, ASCII digits
    /// or `_`, and of each other char.
    pub(super) letter_start: u64,
}

const WHITESPACE: u16 = 1 << 0;
const BAD_WHITESPACE: u16 = 1 << 1;
const LINE_BREAK: u16 = 1 << 2;
const COLON_STOP: u16 = 1 << 3;
const ALPHA: u16 = 1 << 4;
const DIGIT: u16 = 1 << 5;
const UNDERSCORE: u16 = 1 << 6;
const HASH: u16 = 1 << 7;

/// The number of classes that [classify] computes per block: those in [CLASSES], and the UTF-8
/// continuation bytes.
const CLASS_COUNT: usize = 9;

/// The classes of an ASCII byte.
const fn ascii_classes(byte: u8) -> u16 {
    let mut classes = 0;
    if matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ') {
        classes |= WHITESPACE | COLON_STOP;
    }
    if matches!(byte, 0x0B | 0x0C) {
        classes |= BAD_WHITESPACE;
    }
    if matches!(byte, b'\n' | b'\r') {
        classes |= LINE_BREAK;
    }
    if byte == b':' {
        classes |= COLON_STOP;
    }
    if byte.is_ascii_alphabetic() {
        classes |= ALPHA;
    }
    if byte.is_ascii_digit() {
        classes |= DIGIT;
    }
    if byte == b'_' {
        classes |= UNDERSCORE;
    }
    if byte == b'#' {
        classes |= HASH;
    }
    classes
}

/// A lookup table for ascii bytes to classes, computed at compile time.
static ASCII_CLASSES: [u16; 128] = {
    let mut table = [0; 128];
    let mut byte = 0;
    while byte < 128 {
        table[byte] = ascii_classes(byte as u8);
        byte += 1;
    }
    table
};

/// The ASCII classes, in the order that the classifiers return them. The UTF-8 continuation bytes
/// come after them.
const CLASSES: [u16; 8] = [
    WHITESPACE,
    BAD_WHITESPACE,
    LINE_BREAK,
    COLON_STOP,
    ALPHA,
    DIGIT,
    UNDERSCORE,
    HASH,
];

#[cfg(any(test, not(target_arch = "x86_64")))]
fn classify_scalar(bytes: &[u8; 64], len: usize) -> [u64; CLASS_COUNT] {
    let mut classes = [0; CLASS_COUNT];
    for (index, &byte) in bytes[..len].iter().enumerate() {
        if !byte.is_ascii() {
            if byte & 0xC0 == 0x80 {
                classes[8] |= 1 << index;
            }
            continue;
        }
        let byte_classes = ASCII_CLASSES[byte as usize];
        for (bitmap, class) in classes.iter_mut().zip(CLASSES) {
            if byte_classes & class != 0 {
                *bitmap |= 1 << index;
            }
        }
    }
    classes
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
    mut classify_block: impl FnMut(&[u8; 64], usize) -> [u64; CLASS_COUNT],
) {
    blocks.clear();
    blocks.reserve(source.len() / 64 + 2);
    // The sequence classes of the last byte of the block before, so that a sequence that crosses
    // a block boundary starts one letter.
    let mut carry = [0u64; 4];
    let mut push = |classes: [u64; CLASS_COUNT], valid: u64| {
        let [
            whitespace,
            bad_whitespace,
            line_break,
            colon_stop,
            alpha,
            digit,
            underscore,
            hash,
            continuation,
        ] = classes;
        let sequences = [whitespace, alpha, digit, underscore];
        let mut starts = !(whitespace | alpha | digit | underscore | continuation);
        for (sequence, carry) in sequences.iter().zip(carry.iter_mut()) {
            starts |= sequence & !((sequence << 1) | *carry);
            *carry = sequence >> 63;
        }
        blocks.push(Block {
            whitespace: whitespace & valid,
            bad_whitespace: bad_whitespace & valid,
            line_break: line_break & valid,
            colon_stop: colon_stop | !valid,
            hash: hash & valid,
            letter_start: starts & valid,
        });
    };
    let (chunks, tail) = source.as_chunks::<64>();
    for bytes in chunks {
        push(classify_block(bytes, 64), u64::MAX);
    }
    if !tail.is_empty() {
        let mut bytes = [0; 64];
        bytes[..tail.len()].copy_from_slice(tail);
        let valid = (1u64 << tail.len()) - 1;
        push(classify_block(&bytes, tail.len()), valid);
    }
    push([0; CLASS_COUNT], 0);
}

#[cfg(target_arch = "x86_64")]
mod x86 {
    use super::{ASCII_CLASSES, Block, CLASS_COUNT, CLASSES, classify_with};
    use std::arch::x86_64::*;

    /// For each class, a bitset of the ASCII bytes in it: bit `byte & 7` of entry `byte >> 3`.
    static BITSETS: [[u8; 16]; 8] = {
        let mut tables = [[0; 16]; 8];
        let mut class = 0;
        while class < 8 {
            let mut byte = 0;
            while byte < 128 {
                if ASCII_CLASSES[byte] & CLASSES[class] != 0 {
                    tables[class][byte >> 3] |= 1 << (byte & 7);
                }
                byte += 1;
            }
            class += 1;
        }
        tables
    };

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
        ];
        let bits = load(&BITS);
        let low_nibble = _mm256_set1_epi8(0x0F);
        let seven = _mm256_set1_epi8(7);
        // Continuation bytes, 0x80 to 0xBF, are the signed bytes below -64.
        let continuation_limit = _mm256_set1_epi8(-64);

        // Looks up the class bitsets of 32 bytes. Non-ASCII bytes alias ASCII entries, so they
        // are cleared.
        let half = |bytes: &[u8]| {
            // SAFETY: `bytes` is 32 bytes long.
            let v = unsafe { _mm256_loadu_si256(bytes.as_ptr().cast()) };
            let index = _mm256_and_si256(_mm256_srli_epi16::<3>(v), low_nibble);
            let bit = _mm256_shuffle_epi8(bits, _mm256_and_si256(v, seven));
            let ascii = !(_mm256_movemask_epi8(v) as u32 as u64);
            let class = |table: __m256i| {
                let hit = _mm256_and_si256(_mm256_shuffle_epi8(table, index), bit);
                (_mm256_movemask_epi8(_mm256_cmpeq_epi8(hit, bit)) as u32 as u64) & ascii
            };
            let continuation =
                _mm256_movemask_epi8(_mm256_cmpgt_epi8(continuation_limit, v)) as u32 as u64;
            [
                class(bitsets[0]),
                class(bitsets[1]),
                class(bitsets[2]),
                class(bitsets[3]),
                class(bitsets[4]),
                class(bitsets[5]),
                class(bitsets[6]),
                class(bitsets[7]),
                continuation,
            ]
        };

        classify_with(source, blocks, |bytes, len| {
            let low = half(&bytes[..32]);
            let high = if len > 32 {
                half(&bytes[32..])
            } else {
                [0; CLASS_COUNT]
            };
            let join = |index: usize| low[index] | (high[index] << 32);
            [
                join(0),
                join(1),
                join(2),
                join(3),
                join(4),
                join(5),
                join(6),
                join(7),
                join(8),
            ]
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
            // Whether each byte is in `first..first + len`, as a signed comparison after moving
            // `first` to `i8::MIN`.
            let range = |first: u8, len: u8| {
                let shifted = _mm_add_epi8(v, _mm_set1_epi8(0x80u8.wrapping_sub(first) as i8));
                _mm_cmplt_epi8(shifted, _mm_set1_epi8(0x80u8.wrapping_add(len) as i8))
            };
            let mask = |v: __m128i| _mm_movemask_epi8(v) as u32 as u64;
            let or = _mm_or_si128;

            let whitespace = or(range(b'\t', 5), eq(b' '));
            [
                mask(whitespace),
                mask(range(0x0B, 2)),
                mask(or(eq(b'\n'), eq(b'\r'))),
                mask(or(whitespace, eq(b':'))),
                mask(or(range(b'a', 26), range(b'A', 26))),
                mask(range(b'0', 10)),
                mask(eq(b'_')),
                mask(eq(b'#')),
                mask(range(0x80, 0x40)),
            ]
        };

        classify_with(source, blocks, |bytes, len| {
            let part = |index: usize| {
                if len > 16 * index {
                    quarter(&bytes[16 * index..16 * (index + 1)])
                } else {
                    [0; CLASS_COUNT]
                }
            };
            let parts = [part(0), part(1), part(2), part(3)];
            std::array::from_fn(|class| {
                parts
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (index, part)| acc | (part[class] << (16 * index)))
            })
        })
    }

    #[cfg(test)]
    mod tests {
        use super::super::{classify_scalar, classify_with};

        fn sample() -> Vec<u8> {
            let mut bytes: Vec<u8> = (0..=255u8).collect();
            bytes.extend_from_slice(
                "hover:bg-red-500/50 [&_p]:m-[calc(1px+2px)] \u{e9}(x)%y @sm:*:flex\r\n\x0B#abz"
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
