//! The `compress` pass, which writes the stream that the parser reads: the non-trivia tokens with
//! their flags, and the trivia between them as the parser's tree takes it.

use super::classify::Block;
use super::scan::{BYTE_TOKENS, KEYWORD_KINDS, keyword};
use super::stream::Token;
use super::{
    BulkTokens, CONTEXTS, DIAGNOSTICS, GAP_DIAGNOSTICS, GAP_SHIFT, KIND_MASK, LINE_BREAK, Pass,
    REGULAR, kind,
};
use crate::lexer::{TextRange, TextSize};
use biome_js_syntax::JsSyntaxKind::*;
use biome_parser::token_source::Trivia;
use biome_rowan::TriviaPieceKind;
use std::mem::MaybeUninit;

impl Pass<'_, '_> {
    /// Writes the stream: the non-trivia tokens and the trivia between them.
    ///
    /// The starts go through in chunks of [CHUNK_BLOCKS] blocks. [Chunk] finds which starts are
    /// trivia, and which flags each token and trivia piece takes from the ones before it, for the
    /// 64 bytes of a block at once. Then one loop writes the tokens of the chunk, and another the
    /// trivia, neither branching on the source.
    #[inline(always)]
    pub(super) fn compress(mut self, extract: impl Extract) -> BulkTokens {
        let overrides = self.split_overrides();
        let count: usize = self
            .blocks
            .iter()
            .map(|block| block.starts.count_ones() as usize)
            .sum();
        let mut writer = Writer::new(self.source, overrides, count);
        let mut chunk = Chunk::default();
        for (index, blocks) in self.blocks.chunks(CHUNK_BLOCKS).enumerate() {
            let base = index * CHUNK_BLOCKS * 64;
            let next_start = self.next_bit(base + blocks.len() * 64, |block| block.starts) as u32;
            chunk.read(blocks, base as u32, next_start, &writer.overrides, extract);
            writer.write_tokens(&chunk);
            writer.write_trivia(&chunk);
        }
        let tokens = writer.finish(&chunk);
        // `carve` and `coalesce` each report in source order.
        self.diagnostics.sort_by_key(|diagnostic| diagnostic.start);
        BulkTokens {
            tokens: tokens.0,
            trivia: tokens.1,
            diagnostics: self.diagnostics,
        }
    }

    /// Returns the kinds of the carved and coalesced tokens that are still tokens, in source
    /// order, split into those of trivia and the others.
    pub(super) fn split_overrides(&mut self) -> Overrides {
        let count = self.carved.len() + self.coalesced.len() + 1;
        let mut overrides = Overrides {
            tokens: Vec::with_capacity(count),
            trivia: Vec::with_capacity(count),
            odd_trivia: Vec::new(),
        };
        let (mut carved, mut coalesced) = (0, 0);
        while carved < self.carved.len() || coalesced < self.coalesced.len() {
            let take_carved = match (self.carved.get(carved), self.coalesced.get(coalesced)) {
                (Some(left), Some(right)) => left.start < right.start,
                (left, _) => left.is_some(),
            };
            let token = if take_carved {
                carved += 1;
                self.carved[carved - 1]
            } else {
                coalesced += 1;
                self.coalesced[coalesced - 1]
            };
            if !self.is_start(token.start as usize) {
                continue;
            }
            let piece = match kind(token.kind) {
                WHITESPACE => PIECE_WHITESPACE,
                NEWLINE => PIECE_NEWLINE,
                COMMENT => PIECE_COMMENT,
                MULTILINE_COMMENT => PIECE_MULTILINE_COMMENT,
                _ => {
                    overrides.tokens.push(token.kind);
                    continue;
                }
            };
            overrides.trivia.push((token.start, piece));
            if token.kind & (CONTEXTS as u16 | DIAGNOSTICS) != REGULAR {
                overrides.odd_trivia.push((token.start, token.kind));
            }
        }
        // The token loop reads one override past the last one, and the trivia loop one past the
        // last trivia override.
        overrides.tokens.push(0);
        overrides.trivia.push((u32::MAX, 0));
        overrides
    }
}

/// The kinds of the tokens that the passes after `classify` found.
pub(super) struct Overrides {
    /// Of the non-trivia tokens, in source order.
    tokens: Vec<u16>,
    /// The start and the code of the [TriviaPieceKind] of the trivia, in source order.
    trivia: Vec<(u32, u8)>,
    /// The start, and the kind and flags, of the trivia whose context flags aren't just
    /// [REGULAR] or that have diagnostics, which [Chunk] doesn't expect.
    odd_trivia: Vec<(u32, u16)>,
}

/// The codes of the trivia piece kinds in [PIECE_KINDS].
const PIECE_WHITESPACE: u8 = 0;
const PIECE_NEWLINE: u8 = 1;
const PIECE_COMMENT: u8 = 2;
const PIECE_MULTILINE_COMMENT: u8 = 3;

static PIECE_KINDS: [TriviaPieceKind; 4] = [
    TriviaPieceKind::Whitespace,
    TriviaPieceKind::Newline,
    TriviaPieceKind::SingleLineComment,
    TriviaPieceKind::MultiLineComment,
];

/// How many blocks `compress` reads at once.
const CHUNK_BLOCKS: usize = 16;
/// How many starts a chunk holds at most, and the room that [Extract] writes past them.
const CHUNK_STARTS: usize = CHUNK_BLOCKS * 64 + 64;

/// Writes the offsets of the bits of a bitmap to an array.
pub(super) trait Extract: Copy {
    /// A byte of flags for each byte of a block.
    type Flags;

    /// Returns the flags whose bit `n` at each byte is the bit at the same offset in
    /// `masks[n]`.
    fn flags(self, masks: [u64; 4]) -> Self::Flags;

    /// Writes the offsets of the bits in `bits`, plus `base`, to `out`, and returns how many
    /// there are. May write up to 64 offsets in all.
    fn extract(self, bits: u64, base: u32, out: &mut [u32]) -> usize;

    /// [Self::extract], which also writes the flags at the offsets to `out_flags`. May write up
    /// to 64 flags in all.
    fn extract_flagged(
        self,
        bits: u64,
        base: u32,
        flags: &Self::Flags,
        out: &mut [u32],
        out_flags: &mut [u8],
    ) -> usize;
}

/// Finds the bits one by one.
#[derive(Clone, Copy)]
pub(super) struct ScalarExtract;

/// The bits of each byte value, one in each byte.
static SPREAD: [u64; 256] = {
    let mut table = [0; 256];
    let mut value = 0;
    while value < 256 {
        let mut bit = 0;
        while bit < 8 {
            if value & (1 << bit) != 0 {
                table[value] |= 1 << (8 * bit);
            }
            bit += 1;
        }
        value += 1;
    }
    table
};

impl Extract for ScalarExtract {
    /// The flags of the eight bytes from each multiple of eight, and zero for the offsets past
    /// the last bit.
    type Flags = [u64; 9];

    #[inline(always)]
    fn flags(self, masks: [u64; 4]) -> [u64; 9] {
        let mut flags = [0; 9];
        for (index, word) in flags[..8].iter_mut().enumerate() {
            let byte = |mask: u64| SPREAD[((mask >> (8 * index)) & 0xFF) as usize];
            *word =
                byte(masks[0]) | byte(masks[1]) << 1 | byte(masks[2]) << 2 | byte(masks[3]) << 3;
        }
        flags
    }

    /// Writes up to seven more offsets after the bits, so that it branches once per eight.
    #[inline(always)]
    fn extract(self, mut bits: u64, base: u32, out: &mut [u32]) -> usize {
        let count = bits.count_ones() as usize;
        let len = count.next_multiple_of(8);
        let out = &mut out[..len];
        let mut index = 0;
        while index < len {
            for slot in 0..8 {
                // SAFETY: `index + slot` is less than `len`, a multiple of eight.
                unsafe { *out.get_unchecked_mut(index + slot) = base + bits.trailing_zeros() };
                bits &= bits.wrapping_sub(1);
            }
            index += 8;
        }
        count
    }

    #[inline(always)]
    fn extract_flagged(
        self,
        mut bits: u64,
        base: u32,
        flags: &[u64; 9],
        out: &mut [u32],
        out_flags: &mut [u8],
    ) -> usize {
        let count = bits.count_ones() as usize;
        let len = count.next_multiple_of(8);
        let (out, out_flags) = (&mut out[..len], &mut out_flags[..len]);
        let mut index = 0;
        while index < len {
            for slot in 0..8 {
                // Past the last bit, `bit` is 64, whose flags are zero.
                let bit = bits.trailing_zeros();
                let flag = (flags[(bit / 8) as usize] >> (8 * (bit % 8))) as u8;
                // SAFETY: `index + slot` is less than `len`, a multiple of eight.
                unsafe {
                    *out.get_unchecked_mut(index + slot) = base + bit;
                    *out_flags.get_unchecked_mut(index + slot) = flag;
                }
                bits &= bits.wrapping_sub(1);
            }
            index += 8;
        }
        count
    }
}

#[cfg(target_arch = "x86_64")]
pub(super) use x86::Avx512Extract;

#[cfg(target_arch = "x86_64")]
mod x86 {
    use super::Extract;
    use std::arch::x86_64::*;

    /// Compresses the offsets of the bits with one AVX-512 instruction, rather than a loop that
    /// its branch predictor doesn't foresee the end of. Only exists on CPUs with AVX-512 F, BW
    /// and VBMI2.
    #[derive(Clone, Copy)]
    pub(in super::super) struct Avx512Extract(());

    impl Avx512Extract {
        pub(in super::super) fn new() -> Option<Self> {
            (std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512bw")
                && std::arch::is_x86_feature_detected!("avx512vbmi2"))
            .then_some(Self(()))
        }
    }

    /// The offsets of the bytes of a 64-byte vector.
    static OFFSETS: [u8; 64] = {
        let mut offsets = [0; 64];
        let mut index = 0;
        while index < 64 {
            offsets[index] = index as u8;
            index += 1;
        }
        offsets
    };

    impl Extract for Avx512Extract {
        type Flags = __m512i;

        #[inline(always)]
        fn flags(self, masks: [u64; 4]) -> __m512i {
            // SAFETY: the CPU supports AVX-512 F and BW, as `Self` exists.
            unsafe {
                _mm512_or_si512(
                    _mm512_or_si512(
                        _mm512_maskz_set1_epi8(masks[0], 1),
                        _mm512_maskz_set1_epi8(masks[1], 2),
                    ),
                    _mm512_or_si512(
                        _mm512_maskz_set1_epi8(masks[2], 4),
                        _mm512_maskz_set1_epi8(masks[3], 8),
                    ),
                )
            }
        }

        /// Writes 16 offsets, or 64 if there are more than 16 bits.
        #[inline(always)]
        fn extract(self, bits: u64, base: u32, out: &mut [u32]) -> usize {
            let count = bits.count_ones() as usize;
            let out = &mut out[..64];
            // SAFETY: the CPU supports AVX-512 F, BW and VBMI2, as `Self` exists, and `out` has
            // room for 64 offsets.
            unsafe {
                let offsets = _mm512_loadu_si512(OFFSETS.as_ptr().cast());
                let compressed = _mm512_maskz_compress_epi8(bits, offsets);
                let base = _mm512_set1_epi32(base as i32);
                let mut write = |index: usize, part: __m128i| {
                    let part = _mm512_add_epi32(_mm512_cvtepu8_epi32(part), base);
                    _mm512_storeu_si512(out.as_mut_ptr().add(16 * index).cast(), part);
                };
                write(0, _mm512_castsi512_si128(compressed));
                if count > 16 {
                    write(1, _mm512_extracti32x4_epi32::<1>(compressed));
                    write(2, _mm512_extracti32x4_epi32::<2>(compressed));
                    write(3, _mm512_extracti32x4_epi32::<3>(compressed));
                }
            }
            count
        }

        #[inline(always)]
        fn extract_flagged(
            self,
            bits: u64,
            base: u32,
            flags: &__m512i,
            out: &mut [u32],
            out_flags: &mut [u8],
        ) -> usize {
            let out_flags = &mut out_flags[..64];
            // SAFETY: as in `extract`, and `out_flags` has room for 64 bytes.
            unsafe {
                let compressed = _mm512_maskz_compress_epi8(bits, *flags);
                _mm512_storeu_si512(out_flags.as_mut_ptr().cast(), compressed);
            }
            self.extract(bits, base, out)
        }
    }
}

/// The flags of a token that [Chunk] finds: it has an override.
const OVERRIDDEN: u8 = 1;
/// A non-trivia token with a line break in the trivia before it.
const TOKEN_LINE_BREAK: u8 = 2;
/// A non-trivia token with trivia before it.
const TOKEN_AFTER_TRIVIA: u8 = 4;
/// A non-trivia token that may be a keyword.
const TOKEN_KEYWORD: u8 = 8;
/// A newline. Its code in [PIECE_KINDS] is this flag shifted down.
const TRIVIA_NEWLINE: u8 = 2;
/// Trivia after a non-trivia token, with no newline between them.
const TRIVIA_TRAILING: u8 = 4;

const _: () = assert!(TRIVIA_NEWLINE >> 1 == PIECE_NEWLINE);

/// The tokens and the trivia of a chunk, with their flags, and the carries of those flags from
/// one block to the next.
struct Chunk {
    /// The starts, the ends and the flags of the non-trivia tokens of the chunk.
    tokens: [u32; CHUNK_STARTS],
    token_ends: [u32; CHUNK_STARTS],
    token_flags: [u8; CHUNK_STARTS],
    token_count: usize,
    /// The starts, the ends and the flags of the trivia of the chunk.
    trivia: [u32; CHUNK_STARTS],
    trivia_ends: [u32; CHUNK_STARTS],
    trivia_flags: [u8; CHUNK_STARTS],
    trivia_count: usize,
    /// Trivia since the last non-trivia token.
    after_trivia_carry: bool,
    /// A line break since the last non-trivia token.
    line_break_carry: bool,
    /// No newline since the last non-trivia token.
    trailing_carry: bool,
    next_trivia_override: usize,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            tokens: [0; CHUNK_STARTS],
            token_ends: [0; CHUNK_STARTS],
            token_flags: [0; CHUNK_STARTS],
            token_count: 0,
            trivia: [0; CHUNK_STARTS],
            trivia_ends: [0; CHUNK_STARTS],
            trivia_flags: [0; CHUNK_STARTS],
            trivia_count: 0,
            after_trivia_carry: false,
            line_break_carry: false,
            trailing_carry: false,
            next_trivia_override: 0,
        }
    }
}

impl Chunk {
    /// Reads `blocks`, which start at `base` and are followed by a start at `next_start`.
    ///
    /// A carry that runs through the bitmap of a block finds what comes before each token: added
    /// to the bitmap of the bytes that aren't non-trivia token starts, a bit at a trivia start
    /// carries up to the next non-trivia token start, and sets its bit. In the same way, the bits
    /// after the starts of tokens carry up to the next starts, their ends.
    #[inline(always)]
    fn read(
        &mut self,
        blocks: &[Block],
        base: u32,
        next_start: u32,
        overrides: &Overrides,
        extract: impl Extract,
    ) {
        // The trivia overrides of the chunk.
        let mut trivia = [0u64; CHUNK_BLOCKS];
        let mut newline = [0u64; CHUNK_BLOCKS];
        let mut comment_break = [0u64; CHUNK_BLOCKS];
        let end = base + (blocks.len() * 64) as u32;
        let mut next = self.next_trivia_override;
        while overrides.trivia[next].0 < end {
            let (start, piece) = overrides.trivia[next];
            let offset = start - base;
            let (index, bit) = ((offset / 64) as usize, 1u64 << (offset % 64));
            trivia[index] |= bit;
            newline[index] |= bit * u64::from(piece == PIECE_NEWLINE);
            comment_break[index] |= bit * u64::from(piece == PIECE_MULTILINE_COMMENT);
            next += 1;
        }
        self.next_trivia_override = next;

        let (mut token_count, mut trivia_count) = (0, 0);
        let (mut token_end_count, mut trivia_end_count) = (0, 0);
        // A token at the end of a block, whose end is in a later one.
        let (mut token_end_carry, mut trivia_end_carry) = (false, false);
        for (index, block) in blocks.iter().enumerate() {
            let starts = block.starts;
            let overridden = block.overridden & starts;
            let trivia_starts = starts & ((block.whitespace & !overridden) | trivia[index]);
            let tokens = starts & !trivia_starts;
            let newlines = starts & ((block.line_break & !overridden) | newline[index]);
            let breaks = newlines | comment_break[index];
            let (line_break, carry) = carry_to(tokens, breaks, self.line_break_carry);
            self.line_break_carry = carry;
            let (after_trivia, carry) = carry_to(tokens, trivia_starts, self.after_trivia_carry);
            self.after_trivia_carry = carry;
            // A non-trivia token start carries through the bytes that don't start a newline.
            let not_newline = !newlines;
            let (sum, carry_low) = not_newline.overflowing_add(tokens);
            let (sum, carry_high) = sum.overflowing_add(u64::from(self.trailing_carry));
            self.trailing_carry = carry_low | carry_high;
            let carried_into = sum ^ not_newline ^ tokens;

            let (token_ends, carry) = carry_to_next(starts, tokens, token_end_carry);
            token_end_carry = carry;
            let (trivia_ends, carry) = carry_to_next(starts, trivia_starts, trivia_end_carry);
            trivia_end_carry = carry;

            let block_base = base + (index * 64) as u32;
            // The flags of tokens and trivia, which are at different bytes.
            let flags = extract.flags([
                overridden,
                line_break | newlines,
                after_trivia | (carried_into & trivia_starts & !newlines),
                block.lower_pair & tokens & !overridden,
            ]);
            token_count += extract.extract_flagged(
                tokens,
                block_base,
                &flags,
                &mut self.tokens[token_count..],
                &mut self.token_flags[token_count..],
            );
            token_end_count += extract.extract(
                token_ends,
                block_base,
                &mut self.token_ends[token_end_count..],
            );
            trivia_count += extract.extract_flagged(
                trivia_starts,
                block_base,
                &flags,
                &mut self.trivia[trivia_count..],
                &mut self.trivia_flags[trivia_count..],
            );
            trivia_end_count += extract.extract(
                trivia_ends,
                block_base,
                &mut self.trivia_ends[trivia_end_count..],
            );
        }
        // The last token of the chunk ends at the next start, if no start of the chunk ends it.
        self.token_ends[token_end_count] = next_start;
        self.trivia_ends[trivia_end_count] = next_start;
        self.token_count = token_count;
        self.trivia_count = trivia_count;
    }
}

/// Adds the bits of `from`, which aren't in `to`, to the bitmap of the bytes that aren't in `to`,
/// plus `carry` at the first byte. Returns the bits of `to` that a carry reached, which are
/// those after a bit of `from` and no other bit of `to`, and whether a carry runs past the block.
#[inline(always)]
fn carry_to(to: u64, from: u64, carry: bool) -> (u64, bool) {
    let (sum, carry_low) = (!to).overflowing_add(from);
    let (sum, carry_high) = sum.overflowing_add(u64::from(carry));
    (sum & to, carry_low | carry_high)
}

/// Returns the next bit of `starts` after each bit of `from`, which is a subset of `starts`,
/// with `carry` for a bit before the block. Returns too whether the next bit of the last bit
/// of `from` is after the block.
#[inline(always)]
fn carry_to_next(starts: u64, from: u64, carry: bool) -> (u64, bool) {
    // The bit after each bit of `from` carries through the bytes that aren't starts, up to the
    // next start. No two carry to the same start, as each bit of `from` is a start.
    let (sum, carry_low) = (!starts).overflowing_add(from << 1);
    let (sum, carry_high) = sum.overflowing_add(u64::from(carry));
    (sum & starts, carry_low || carry_high || from >> 63 != 0)
}

/// Writes the tokens and the trivia that `compress` reads.
struct Writer<'a> {
    source: &'a [u8],
    overrides: Overrides,
    next_override: usize,
    next_trivia_override: usize,
    tokens: Vec<Token>,
    trivia: Vec<Trivia>,
}

impl<'a> Writer<'a> {
    /// A writer for up to `count` tokens and trivia, and `EOF`.
    fn new(source: &'a [u8], overrides: Overrides, count: usize) -> Self {
        Self {
            source,
            overrides,
            next_override: 0,
            next_trivia_override: 0,
            tokens: Vec::with_capacity(count + 1),
            trivia: Vec::with_capacity(count),
        }
    }

    /// Writes the non-trivia tokens of `chunk`.
    #[inline(always)]
    fn write_tokens(&mut self, chunk: &Chunk) {
        let starts = &chunk.tokens[..chunk.token_count];
        let ends = &chunk.token_ends[..chunk.token_count];
        let flags = &chunk.token_flags[..chunk.token_count];
        let first = self.tokens.len();
        assert!(first + starts.len() <= self.tokens.capacity());
        let source = self.source;
        let overrides = &self.overrides.tokens;
        let mut next_override = self.next_override;
        let out = self.tokens.as_mut_ptr();
        let mut candidates = [MaybeUninit::<u32>::uninit(); CHUNK_STARTS];
        let mut candidate_count = 0;
        for index in 0..starts.len() {
            let (start, end, flag) = (starts[index], ends[index], flags[index]);
            // Pick the override or the kind of the first byte without a branch, as either is
            // common.
            let overridden = usize::from(flag & OVERRIDDEN);
            // SAFETY: starts are offsets in the source.
            let by_byte = BYTE_TOKENS[unsafe { *source.get_unchecked(start as usize) } as usize];
            // SAFETY: every overridden token has an override, and one more follows them.
            let by_override = unsafe { *overrides.get_unchecked(next_override) };
            next_override += overridden;
            let raw = if overridden != 0 {
                by_override
            } else {
                by_byte
            };
            let line_break = u32::from(flag & TOKEN_LINE_BREAK != 0);
            let after_trivia = u32::from(flag & TOKEN_AFTER_TRIVIA != 0);
            // The trivia before the token is regular, as `split_overrides` checks.
            let gap = GAP_ALL ^ (after_trivia * (GAP_ALL ^ GAP_REGULAR));
            // SAFETY: the vector has room for the tokens of the chunk after the ones it has.
            unsafe {
                out.add(first + index).write(Token {
                    word: u32::from(raw) | (line_break * LINE_BREAK) | gap,
                    start,
                    end,
                });
            }
            // SAFETY: the count counts at most the tokens of the chunk.
            unsafe { candidates.get_unchecked_mut(candidate_count) }.write((first + index) as u32);
            candidate_count += usize::from(flag & TOKEN_KEYWORD != 0);
        }
        // SAFETY: the loop initialized them.
        unsafe { self.tokens.set_len(first + starts.len()) };
        self.next_override = next_override;
        for candidate in &candidates[..candidate_count] {
            // SAFETY: the loop initialized the candidates up to the count, which are indices of
            // tokens it wrote.
            let token = unsafe {
                self.tokens
                    .get_unchecked_mut(candidate.assume_init() as usize)
            };
            let code = keyword(source, token.start as usize, token.end as usize);
            token.word = (token.word & !u32::from(KIND_MASK))
                | u32::from(KEYWORD_KINDS[code as usize] & KIND_MASK);
        }
    }

    /// Writes the trivia of `chunk`.
    #[inline(always)]
    fn write_trivia(&mut self, chunk: &Chunk) {
        let starts = &chunk.trivia[..chunk.trivia_count];
        let ends = &chunk.trivia_ends[..chunk.trivia_count];
        let flags = &chunk.trivia_flags[..chunk.trivia_count];
        let first = self.trivia.len();
        assert!(first + starts.len() <= self.trivia.capacity());
        let overrides = &self.overrides.trivia;
        let mut next_override = self.next_trivia_override;
        let out = self.trivia.as_mut_ptr();
        for index in 0..starts.len() {
            let (start, end, flag) = (starts[index], ends[index], flags[index]);
            let overridden = usize::from(flag & OVERRIDDEN);
            let newline = (flag & TRIVIA_NEWLINE) >> 1;
            // SAFETY: every overridden trivia piece has an override, and one more follows them.
            let by_override = unsafe { overrides.get_unchecked(next_override) }.1;
            next_override += overridden;
            let piece = if overridden != 0 {
                by_override
            } else {
                newline
            };
            let trailing = flag & TRIVIA_TRAILING != 0;
            // SAFETY: the vector has room for the trivia of the chunk after the ones it has.
            unsafe {
                out.add(first + index).write(Trivia::new(
                    PIECE_KINDS[usize::from(piece & 3)],
                    TextRange::new(TextSize::from(start), TextSize::from(end)),
                    trailing,
                ));
            }
        }
        // SAFETY: the loop initialized them.
        unsafe { self.trivia.set_len(first + starts.len()) };
        self.next_trivia_override = next_override;
    }

    /// Adds `EOF`, after the trivia that `chunk`, the last one, carries, gives the tokens after
    /// odd trivia their flags, and returns the tokens and the trivia.
    fn finish(mut self, chunk: &Chunk) -> (Vec<Token>, Vec<Trivia>) {
        let len = self.source.len() as u32;
        let gap = if chunk.after_trivia_carry {
            GAP_REGULAR
        } else {
            GAP_ALL
        };
        let line_break = if chunk.line_break_carry {
            LINE_BREAK
        } else {
            0
        };
        self.tokens.push(Token {
            word: EOF as u32 | u32::from(REGULAR) | gap | line_break,
            start: len,
            end: len,
        });
        for &(start, raw) in &self.overrides.odd_trivia {
            let index = self.tokens.partition_point(|token| token.start <= start);
            let word = &mut self.tokens[index].word;
            *word &= !((CONTEXTS & !u32::from(raw)) << GAP_SHIFT);
            if raw & DIAGNOSTICS != 0 {
                *word |= GAP_DIAGNOSTICS;
            }
        }
        (self.tokens, self.trivia)
    }
}

/// The flags of a token after no trivia, which fits every context.
const GAP_ALL: u32 = CONTEXTS << GAP_SHIFT;
/// The flags of a token after regular trivia.
const GAP_REGULAR: u32 = (REGULAR as u32) << GAP_SHIFT;
