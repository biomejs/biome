//! Character classes of the source bytes, computed 64 bytes at a time.
//!
//! Each class is a bitmap per 64-byte block, where bit `i` describes byte `i` of the block.
//! x86_64 uses AVX2 when the CPU has it and SSE2 otherwise; other targets classify one byte at a
//! time with a lookup table.

/// The class bitmaps of one 64-byte block, plus the token starts derived from them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Block {
    /// Bytes that start a token, assuming every token is an identifier, a number, consecutive
    /// whitespace, a line break or a single punctuator. The later passes correct this for
    /// literals, comments and operators longer than one byte.
    pub(super) starts: u64,
    /// `\n` and `\r`.
    pub(super) line_break: u64,
    /// Bytes of non-ASCII chars.
    pub(super) non_ascii: u64,
    /// Bytes that need a closer look than their class gives: quotes, the backtick, `/`, `\`, `#`,
    /// and the control chars the lexer rejects.
    pub(super) special: u64,
    /// `{` and `}`.
    pub(super) brace: u64,
    /// Bytes that may start a number or an operator longer than one byte: digits, and the
    /// first bytes of those operators followed by a byte that can continue one.
    pub(super) merge: u64,
    /// `a` to `z` followed by `a` to `z`, as every keyword starts.
    pub(super) lower_pair: u64,
    /// Token starts whose kind the passes after `classify` decided.
    pub(super) overridden: u64,
}

const WORD: u16 = 1 << 0;
const SPACE: u16 = 1 << 1;
const LF: u16 = 1 << 2;
const CR: u16 = 1 << 3;
const DIGIT: u16 = 1 << 4;
const LOWER: u16 = 1 << 5;
const SPECIAL: u16 = 1 << 6;
const BRACE: u16 = 1 << 7;
const OPERATOR: u16 = 1 << 8;
const OPERATOR_NEXT: u16 = 1 << 9;

/// The classes of an ASCII byte.
const fn ascii_classes(byte: u8) -> u16 {
    let mut classes = 0;
    if byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$' {
        classes |= WORD;
    }
    if matches!(byte, b'\t' | 0x0B | 0x0C | b' ') {
        classes |= SPACE;
    }
    if byte == b'\n' {
        classes |= LF;
    }
    if byte == b'\r' {
        classes |= CR;
    }
    if byte.is_ascii_digit() {
        classes |= DIGIT | OPERATOR_NEXT;
    }
    if byte.is_ascii_lowercase() {
        classes |= LOWER;
    }
    if matches!(byte, b'"' | b'\'' | b'`' | b'/' | b'\\' | b'#' | 0x7F)
        || (byte < 0x20 && !matches!(byte, b'\t'..=b'\r'))
    {
        classes |= SPECIAL;
    }
    if matches!(byte, b'{' | b'}') {
        classes |= BRACE;
    }
    if matches!(
        byte,
        b'!' | b'%' | b'&' | b'*' | b'+' | b'-' | b'.' | b'<' | b'=' | b'?' | b'^' | b'|'
    ) {
        classes |= OPERATOR;
    }
    if matches!(
        byte,
        b'&' | b'*' | b'+' | b'-' | b'.' | b'<' | b'=' | b'>' | b'?' | b'|'
    ) {
        classes |= OPERATOR_NEXT;
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

/// The classes of the 64 bytes of a block, before token starts are derived.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Classes {
    word: u64,
    space: u64,
    lf: u64,
    cr: u64,
    digit: u64,
    lower: u64,
    special: u64,
    brace: u64,
    operator: u64,
    operator_next: u64,
    non_ascii: u64,
}

#[cfg(any(test, not(target_arch = "x86_64")))]
fn classify_scalar(bytes: &[u8; 64]) -> Classes {
    let mut classes = Classes::default();
    for (index, &byte) in bytes.iter().enumerate() {
        let bit = 1u64 << index;
        if byte >= 0x80 {
            classes.non_ascii |= bit;
            continue;
        }
        let byte_classes = ASCII_CLASSES[byte as usize];
        let set = |class: u16| if byte_classes & class != 0 { bit } else { 0 };
        classes.word |= set(WORD);
        classes.space |= set(SPACE);
        classes.lf |= set(LF);
        classes.cr |= set(CR);
        classes.digit |= set(DIGIT);
        classes.lower |= set(LOWER);
        classes.special |= set(SPECIAL);
        classes.brace |= set(BRACE);
        classes.operator |= set(OPERATOR);
        classes.operator_next |= set(OPERATOR_NEXT);
    }
    classes
}

/// Combines the classes of consecutive blocks into [Block]s, carrying the state that token
/// starts depend on across block boundaries.
#[derive(Default)]
struct StartTracker {
    previous_word: u64,
    previous_space: u64,
    previous_cr: u64,
    /// Whether the last byte of the previous block is an operator byte.
    previous_operator: u64,
    /// Whether the last byte of the previous block is a lowercase letter.
    previous_lower: u64,
}

impl StartTracker {
    /// Returns the block of `classes`, and the bits to add to the `merge` and the `lower_pair` of
    /// the previous block.
    #[inline(always)]
    fn block(&mut self, classes: Classes, valid: u64) -> (Block, u64, u64) {
        let word = (classes.word | classes.non_ascii) & valid;
        let space = classes.space & valid;
        let lf = classes.lf & valid;
        let cr = classes.cr & valid;
        let previous_word = (word << 1) | self.previous_word;
        let previous_space = (space << 1) | self.previous_space;
        let previous_cr = (cr << 1) | self.previous_cr;
        self.previous_word = word >> 63;
        self.previous_space = space >> 63;
        self.previous_cr = cr >> 63;
        let previous_merge = (self.previous_operator & classes.operator_next & 1) << 63;
        self.previous_operator = classes.operator >> 63;
        let previous_lower_pair = (self.previous_lower & classes.lower & 1) << 63;
        self.previous_lower = classes.lower >> 63;

        // Consecutive word bytes or spaces are one token; `\r\n` is one line break; every other
        // byte starts a token of its own.
        let other = valid & !(word | space | lf | cr);
        let starts =
            (word & !previous_word) | (space & !previous_space) | cr | (lf & !previous_cr) | other;

        let block = Block {
            starts,
            line_break: lf | cr,
            non_ascii: classes.non_ascii & valid,
            special: classes.special & valid,
            brace: classes.brace & valid,
            merge: (classes.digit | (classes.operator & (classes.operator_next >> 1))) & valid,
            lower_pair: classes.lower & (classes.lower >> 1) & valid,
            overridden: 0,
        };
        (block, previous_merge, previous_lower_pair)
    }
}

/// Classifies `source` into one [Block] per 64 bytes, plus one empty trailing block.
pub(super) fn classify(source: &[u8]) -> Vec<Block> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: the CPU supports AVX2.
            return unsafe { x86::classify_avx2(source) };
        }
        // SAFETY: SSE2 is part of the x86_64 baseline.
        unsafe { x86::classify_sse2(source) }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        classify_with(source, classify_scalar)
    }
}

/// Classifies `source` with `classify_block`, which receives each block's 64 bytes; the last
/// block is padded with zeros.
#[inline(always)]
fn classify_with(
    source: &[u8],
    mut classify_block: impl FnMut(&[u8; 64]) -> Classes,
) -> Vec<Block> {
    let block_count = source.len().div_ceil(64);
    let mut blocks: Vec<Block> = Vec::with_capacity(block_count + 1);
    let mut tracker = StartTracker::default();
    let mut push = |(block, previous_merge, previous_lower_pair): (Block, u64, u64)| {
        if let Some(previous) = blocks.last_mut() {
            previous.merge |= previous_merge;
            previous.lower_pair |= previous_lower_pair;
        }
        blocks.push(block);
    };
    let (chunks, tail) = source.as_chunks::<64>();
    for bytes in chunks {
        push(tracker.block(classify_block(bytes), u64::MAX));
    }
    if !tail.is_empty() {
        let mut bytes = [0; 64];
        bytes[..tail.len()].copy_from_slice(tail);
        let valid = (1u64 << tail.len()) - 1;
        push(tracker.block(classify_block(&bytes), valid));
    }
    blocks.push(Block::default());
    blocks
}

#[cfg(target_arch = "x86_64")]
mod x86 {
    use super::{
        ASCII_CLASSES, BRACE, Block, Classes, DIGIT, LOWER, OPERATOR, OPERATOR_NEXT, SPACE,
        SPECIAL, WORD, classify_with,
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

    static BITSETS: [[u8; 16]; 8] = [
        bitset(WORD),
        bitset(SPACE),
        bitset(DIGIT),
        bitset(LOWER),
        bitset(SPECIAL),
        bitset(BRACE),
        bitset(OPERATOR),
        bitset(OPERATOR_NEXT),
    ];

    static BITS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

    /// # Safety
    /// The CPU must support AVX2.
    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn classify_avx2(source: &[u8]) -> Vec<Block> {
        // SAFETY: the tables are 16 bytes long.
        let load = |table: &[u8; 16]| unsafe {
            _mm256_broadcastsi128_si256(_mm_loadu_si128(table.as_ptr().cast()))
        };
        let bitsets = BITSETS.each_ref().map(load);
        let bits = load(&BITS);
        let low_nibble = _mm256_set1_epi8(0x0F);
        let seven = _mm256_set1_epi8(7);
        let lf = _mm256_set1_epi8(b'\n' as i8);
        let cr = _mm256_set1_epi8(b'\r' as i8);

        // Looks up the class bitsets of 32 bytes. Non-ASCII bytes alias ASCII entries, so the
        // caller clears them.
        let half = |bytes: &[u8]| {
            // SAFETY: `bytes` is 32 bytes long.
            let v = unsafe { _mm256_loadu_si256(bytes.as_ptr().cast()) };
            let index = _mm256_and_si256(_mm256_srli_epi16::<3>(v), low_nibble);
            let bit = _mm256_shuffle_epi8(bits, _mm256_and_si256(v, seven));
            let class = |table: __m256i| {
                let hit = _mm256_and_si256(_mm256_shuffle_epi8(table, index), bit);
                _mm256_movemask_epi8(_mm256_cmpeq_epi8(hit, bit)) as u32 as u64
            };
            let non_ascii = _mm256_movemask_epi8(v) as u32 as u64;
            let ascii = !non_ascii;
            [
                class(bitsets[0]) & ascii,
                class(bitsets[1]) & ascii,
                class(bitsets[2]) & ascii,
                class(bitsets[3]) & ascii,
                class(bitsets[4]) & ascii,
                class(bitsets[5]) & ascii,
                class(bitsets[6]) & ascii,
                class(bitsets[7]) & ascii,
                _mm256_movemask_epi8(_mm256_cmpeq_epi8(v, lf)) as u32 as u64,
                _mm256_movemask_epi8(_mm256_cmpeq_epi8(v, cr)) as u32 as u64,
                non_ascii,
            ]
        };

        classify_with(source, |bytes| {
            let low = half(&bytes[..32]);
            let high = half(&bytes[32..]);
            let join = |index: usize| low[index] | (high[index] << 32);
            Classes {
                word: join(0),
                space: join(1),
                digit: join(2),
                lower: join(3),
                special: join(4),
                brace: join(5),
                operator: join(6),
                operator_next: join(7),
                lf: join(8),
                cr: join(9),
                non_ascii: join(10),
            }
        })
    }

    /// # Safety
    /// The CPU must support SSE2, which every x86_64 CPU does.
    #[target_feature(enable = "sse2")]
    pub(super) unsafe fn classify_sse2(source: &[u8]) -> Vec<Block> {
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
                _mm_cmplt_epi8(shifted, _mm_set1_epi8((0x80u8.wrapping_add(len)) as i8))
            };
            let mask = |v: __m128i| _mm_movemask_epi8(v) as u32 as u64;

            let digit = range(b'0', 10);
            let lower = range(b'a', 26);
            let upper = range(b'A', 26);
            let control = _mm_andnot_si128(range(b'\t', 5), range(0, 0x20));
            let word = _mm_or_si128(_mm_or_si128(digit, lower), _mm_or_si128(upper, any(b"_$")));
            Classes {
                word: mask(word),
                space: mask(any(b"\t\x0B\x0C ")),
                lf: mask(eq(b'\n')),
                cr: mask(eq(b'\r')),
                digit: mask(digit),
                lower: mask(lower),
                special: mask(_mm_or_si128(control, any(b"\"'`/\\#\x7F"))),
                brace: mask(any(b"{}")),
                operator: mask(any(b"!%&*+-.<=?^|")),
                operator_next: mask(_mm_or_si128(digit, any(b"&*+-.<=>?|"))),
                non_ascii: mask(v),
            }
        };

        classify_with(source, |bytes| {
            let parts = [
                quarter(&bytes[..16]),
                quarter(&bytes[16..32]),
                quarter(&bytes[32..48]),
                quarter(&bytes[48..]),
            ];
            let join = |field: fn(&Classes) -> u64| {
                parts
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (index, part)| acc | (field(part) << (16 * index)))
            };
            Classes {
                word: join(|c| c.word),
                space: join(|c| c.space),
                lf: join(|c| c.lf),
                cr: join(|c| c.cr),
                digit: join(|c| c.digit),
                lower: join(|c| c.lower),
                special: join(|c| c.special),
                brace: join(|c| c.brace),
                operator: join(|c| c.operator),
                operator_next: join(|c| c.operator_next),
                non_ascii: join(|c| c.non_ascii),
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::super::{classify_scalar, classify_with};

        fn sample() -> Vec<u8> {
            let mut bytes: Vec<_> = (0..=255u8).collect();
            bytes.extend_from_slice("let a = `x${b}` /* é */ // \u{2028}\r\n\t'\\'".as_bytes());
            bytes.extend((0..300u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 7) as u8));
            bytes
        }

        #[test]
        fn simd_classes_match_scalar() {
            let source = sample();
            for len in 0..source.len() {
                let source = &source[..len];
                let expected = classify_with(source, classify_scalar);
                // SAFETY: SSE2 is part of the x86_64 baseline.
                assert_eq!(
                    unsafe { super::classify_sse2(source) },
                    expected,
                    "sse2, len {len}"
                );
                if std::arch::is_x86_feature_detected!("avx2") {
                    // SAFETY: the CPU supports AVX2.
                    assert_eq!(
                        unsafe { super::classify_avx2(source) },
                        expected,
                        "avx2, len {len}"
                    );
                }
            }
        }
    }
}
