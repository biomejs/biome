//! `LineIndex` maps flat `TextSize` offsets into `(Line, Column)`
//! representation.

use std::mem;

use biome_rowan::TextSize;
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
    pub fn new(text: &str) -> LineIndex {
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

        LineIndex {
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
