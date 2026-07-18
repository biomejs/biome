//! `LineIndex` maps flat `TextSize` offsets into `(Line, Column)`
//! representation.

use std::mem;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use biome_text_size::TextSize;
use rustc_hash::FxHashMap;

use crate::{LineCol, WideChar, WideEncoding, WideLineCol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineIndex {
    /// Offset the beginning of each line, zero-based.
    pub newlines: Vec<TextSize>,
    /// List of non-ASCII characters on each line.
    pub line_wide_chars: FxHashMap<u32, Vec<WideChar>>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        Self::new_avx2(text)
            .or_else(|| Self::new_sse42(text))
            .or_else(|| Self::new_neon(text))
            .unwrap_or_else(|| Self::new_scalar(text))
    }

    pub fn new_scalar(text: &str) -> Self {
        let mut line_wide_chars = FxHashMap::default();
        let mut wide_chars = Vec::new();

        let mut newlines = vec![TextSize::from(0)];

        let mut current_col = TextSize::from(0);

        let mut line = 0;
        for (offset, char) in text.char_indices() {
            let char_size = TextSize::of(char);

            if char == '\n' {
                // SAFETY: the conversion from `usize` to `TextSize` can fail if `offset`
                // is larger than 2^32. We don't support such large files.
                let char_offset = TextSize::try_from(offset).expect("TextSize overflow");
                newlines.push(char_offset + char_size);

                // Save any utf-16 characters seen in the previous line
                if !wide_chars.is_empty() {
                    line_wide_chars.insert(line, mem::take(&mut wide_chars));
                }

                // Prepare for processing the next line
                current_col = TextSize::from(0);
                line += 1;
                continue;
            }

            if !char.is_ascii() {
                wide_chars.push(WideChar {
                    start: current_col,
                    end: current_col + char_size,
                });
            }

            current_col += char_size;
        }

        // Save any utf-16 characters seen in the last line
        if !wide_chars.is_empty() {
            line_wide_chars.insert(line, wide_chars);
        }

        Self {
            newlines,
            line_wide_chars,
        }
    }

    /// Builds a line index using an AVX2 fast path for ASCII text spans.
    ///
    /// Returns `None` when the current CPU does not support AVX2, or on non-x86 targets.
    pub fn new_avx2(text: &str) -> Option<Self> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::is_x86_feature_detected!("avx2") {
                // SAFETY: the runtime check above guarantees AVX2 support.
                return Some(unsafe { Self::new_avx2_unchecked(text) });
            }
        }

        None
    }

    /// Builds a line index using an SSE4.2 fast path for ASCII text spans.
    ///
    /// Returns `None` when the current CPU does not support SSE4.2, or on non-x86 targets.
    pub fn new_sse42(text: &str) -> Option<Self> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::is_x86_feature_detected!("sse4.2") {
                // SAFETY: the runtime check above guarantees SSE4.2 support.
                return Some(unsafe { Self::new_sse42_unchecked(text) });
            }
        }

        None
    }

    /// Builds a line index using a NEON fast path for ASCII text spans.
    ///
    /// Returns `None` when the current CPU does not support NEON, or on non-ARM targets.
    pub fn new_neon(_text: &str) -> Option<Self> {
        #[cfg(target_arch = "aarch64")]
        {
            // SAFETY: NEON is part of the baseline AArch64 target features.
            return Some(unsafe { Self::new_neon_unchecked(_text) });
        }

        #[cfg(target_arch = "arm")]
        {
            if std::arch::is_arm_feature_detected!("neon") {
                // SAFETY: the runtime check above guarantees NEON support.
                return Some(unsafe { Self::new_neon_unchecked(_text) });
            }
        }

        None
    }

    #[cfg(target_arch = "aarch64")]
    unsafe fn new_neon_unchecked(text: &str) -> Self {
        unsafe { Self::new_neon_impl(text) }
    }

    #[cfg(target_arch = "arm")]
    #[target_feature(enable = "neon")]
    unsafe fn new_neon_unchecked(text: &str) -> Self {
        unsafe { Self::new_neon_impl(text) }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
    #[cfg_attr(target_arch = "arm", target_feature(enable = "neon"))]
    unsafe fn new_neon_impl(text: &str) -> Self {
        let mut line_wide_chars = FxHashMap::default();
        let mut wide_chars = Vec::new();

        let mut newlines = vec![TextSize::from(0)];
        let mut current_col = TextSize::from(0);
        let mut line = 0;
        let mut offset = 0;
        let bytes = text.as_bytes();

        let newline = unsafe { vdupq_n_u8(b'\n') };
        let high_bit = unsafe { vdupq_n_u8(0x80) };

        while offset + 16 <= bytes.len() {
            let chunk = unsafe { vld1q_u8(bytes.as_ptr().add(offset)) };

            // Mark bytes that need scalar handling: line breaks update `newlines`,
            // and non-ASCII bytes start UTF-8 code points that need wide-char bookkeeping.
            let newline_matches = unsafe { vceqq_u8(chunk, newline) };
            let non_ascii = unsafe { vandq_u8(chunk, high_bit) };
            let interesting = unsafe { vorrq_u8(newline_matches, non_ascii) };

            let Some(ascii_len) = first_nonzero_lane(interesting) else {
                // Entire chunk is ASCII with no newlines, so only the UTF-8 column advances.
                current_col += TextSize::from(16);
                offset += 16;
                continue;
            };

            if ascii_len > 0 {
                // Skip the safe ASCII prefix before the first interesting byte,
                // then handle that one byte/code point scalar and resume SIMD.
                current_col += TextSize::try_from(ascii_len).expect("TextSize overflow");
                offset += ascii_len;
            }

            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        while offset < bytes.len() {
            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        if !wide_chars.is_empty() {
            line_wide_chars.insert(line, wide_chars);
        }

        Self {
            newlines,
            line_wide_chars,
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "sse4.2")]
    unsafe fn new_sse42_unchecked(text: &str) -> Self {
        let mut line_wide_chars = FxHashMap::default();
        let mut wide_chars = Vec::new();

        let mut newlines = vec![TextSize::from(0)];
        let mut current_col = TextSize::from(0);
        let mut line = 0;
        let mut offset = 0;
        let bytes = text.as_bytes();

        let newline = _mm_set1_epi8(b'\n' as i8);

        while offset + 16 <= bytes.len() {
            let chunk = unsafe { _mm_loadu_si128(bytes.as_ptr().add(offset).cast()) };

            // `movemask` gives one bit per byte. Newlines and bytes with the high
            // bit set require scalar handling; all other bytes can be skipped in bulk.
            let interesting_mask = (_mm_movemask_epi8(_mm_cmpeq_epi8(chunk, newline))
                | _mm_movemask_epi8(chunk)) as u32;

            if interesting_mask == 0 {
                // Entire chunk is ASCII with no newlines, so only the UTF-8 column advances.
                current_col += TextSize::from(16);
                offset += 16;
                continue;
            }

            let ascii_len = interesting_mask.trailing_zeros() as usize;
            if ascii_len > 0 {
                // Skip the safe ASCII prefix before the first interesting byte,
                // then handle that one byte/code point scalar and resume SIMD.
                current_col += TextSize::try_from(ascii_len).expect("TextSize overflow");
                offset += ascii_len;
            }

            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        while offset < bytes.len() {
            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        if !wide_chars.is_empty() {
            line_wide_chars.insert(line, wide_chars);
        }

        Self {
            newlines,
            line_wide_chars,
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    unsafe fn new_avx2_unchecked(text: &str) -> Self {
        let mut line_wide_chars = FxHashMap::default();
        let mut wide_chars = Vec::new();

        let mut newlines = vec![TextSize::from(0)];
        let mut current_col = TextSize::from(0);
        let mut line = 0;
        let mut offset = 0;
        let bytes = text.as_bytes();

        let newline = _mm256_set1_epi8(b'\n' as i8);

        while offset + 32 <= bytes.len() {
            let chunk = unsafe { _mm256_loadu_si256(bytes.as_ptr().add(offset).cast()) };

            // `movemask` gives one bit per byte. Newlines and bytes with the high
            // bit set require scalar handling; all other bytes can be skipped in bulk.
            let interesting_mask = (_mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, newline))
                | _mm256_movemask_epi8(chunk)) as u32;

            if interesting_mask == 0 {
                // Entire chunk is ASCII with no newlines, so only the UTF-8 column advances.
                current_col += TextSize::from(32);
                offset += 32;
                continue;
            }

            let ascii_len = interesting_mask.trailing_zeros() as usize;
            if ascii_len > 0 {
                // Skip the safe ASCII prefix before the first interesting byte,
                // then handle that one byte/code point scalar and resume SIMD.
                current_col += TextSize::try_from(ascii_len).expect("TextSize overflow");
                offset += ascii_len;
            }

            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        while offset < bytes.len() {
            let char = text[offset..]
                .chars()
                .next()
                .expect("offset must be within input");

            process_char(
                char,
                offset,
                &mut newlines,
                &mut line_wide_chars,
                &mut wide_chars,
                &mut current_col,
                &mut line,
            );

            offset += char.len_utf8();
        }

        if !wide_chars.is_empty() {
            line_wide_chars.insert(line, wide_chars);
        }

        Self {
            newlines,
            line_wide_chars,
        }
    }

    /// Return the number of lines in the index, clamped to [u32::MAX]
    pub fn len(&self) -> u32 {
        self.newlines.len().try_into().unwrap_or(u32::MAX)
    }

    /// Return `true` if the index contains no lines.
    pub fn is_empty(&self) -> bool {
        self.newlines.is_empty()
    }

    pub fn line_col(&self, offset: TextSize) -> Option<LineCol> {
        let line = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines.get(line)?;
        let col = offset - line_start_offset;

        Some(LineCol {
            line: u32::try_from(line).ok()?,
            col: col.into(),
        })
    }

    pub fn offset(&self, line_col: LineCol) -> Option<TextSize> {
        self.newlines
            .get(line_col.line as usize)
            .map(|offset| offset + TextSize::from(line_col.col))
    }

    pub fn to_wide(&self, enc: WideEncoding, line_col: LineCol) -> Option<WideLineCol> {
        let col = self.utf8_to_wide_col(enc, line_col.line, line_col.col.into());
        Some(WideLineCol {
            line: line_col.line,
            col: u32::try_from(col).ok()?,
        })
    }

    pub fn to_utf8(&self, enc: WideEncoding, line_col: WideLineCol) -> LineCol {
        let col = self.wide_to_utf8_col(enc, line_col.line, line_col.col);
        LineCol {
            line: line_col.line,
            col: col.into(),
        }
    }

    fn utf8_to_wide_col(&self, enc: WideEncoding, line: u32, col: TextSize) -> usize {
        let mut res: usize = col.into();
        if let Some(wide_chars) = self.line_wide_chars.get(&line) {
            for c in wide_chars {
                if c.end <= col {
                    res -= usize::from(c.len()) - c.wide_len(enc);
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }
        res
    }

    fn wide_to_utf8_col(&self, enc: WideEncoding, line: u32, mut col: u32) -> TextSize {
        if let Some(wide_chars) = self.line_wide_chars.get(&line) {
            for c in wide_chars {
                if col > u32::from(c.start) {
                    col += u32::from(c.len()) - c.wide_len(enc) as u32;
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }

        col.into()
    }
}

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm"
))]
fn process_char(
    char: char,
    offset: usize,
    newlines: &mut Vec<TextSize>,
    line_wide_chars: &mut FxHashMap<u32, Vec<WideChar>>,
    wide_chars: &mut Vec<WideChar>,
    current_col: &mut TextSize,
    line: &mut u32,
) {
    let char_size = TextSize::of(char);

    if char == '\n' {
        // SAFETY: the conversion from `usize` to `TextSize` can fail if `offset`
        // is larger than 2^32. We don't support such large files.
        let char_offset = TextSize::try_from(offset).expect("TextSize overflow");
        newlines.push(char_offset + char_size);

        if !wide_chars.is_empty() {
            line_wide_chars.insert(*line, mem::take(wide_chars));
        }

        *current_col = TextSize::from(0);
        *line += 1;
    } else {
        if !char.is_ascii() {
            wide_chars.push(WideChar {
                start: *current_col,
                end: *current_col + char_size,
            });
        }

        *current_col += char_size;
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
fn first_nonzero_lane(vector: uint8x16_t) -> Option<usize> {
    // NEON does not expose an x86-style movemask. Store the 16 lane flags and
    // find the first non-zero lane to locate the first byte needing scalar work.
    let mut lanes = [0u8; 16];
    unsafe { vst1q_u8(lanes.as_mut_ptr(), vector) };
    lanes.iter().position(|lane| *lane != 0)
}
