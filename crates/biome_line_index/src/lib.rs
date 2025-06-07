//! The crate contains tools for converting between byte offsets and line / column positions.

#![deny(clippy::use_self)]

use biome_text_size::TextSize;

mod line_index;

pub use line_index::LineIndex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WideEncoding {
    Utf16,
    Utf32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineCol {
    /// Zero-based
    pub line: u32,
    /// Zero-based utf8 offset
    pub col: u32,
}

/// Deliberately not a generic type and different from `LineCol`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WideLineCol {
    /// Zero-based
    pub line: u32,
    /// Zero-based
    pub col: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct WideChar {
    /// Start offset of a character inside a line, zero-based
    pub start: TextSize,
    /// End offset of a character inside a line, zero-based
    pub end: TextSize,
}

impl WideChar {
    /// Returns the length in 8-bit UTF-8 code units.
    fn len(&self) -> TextSize {
        self.end - self.start
    }

    /// Returns the length in UTF-16 or UTF-32 code units.
    fn wide_len(&self, enc: WideEncoding) -> usize {
        match enc {
            WideEncoding::Utf16 => {
                if self.len() == TextSize::from(4) {
                    2
                } else {
                    1
                }
            }

            WideEncoding::Utf32 => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::WideEncoding::{Utf16, Utf32};
    use crate::WideLineCol;
    use crate::line_index::LineIndex;
    use crate::{LineCol, WideEncoding};
    use biome_text_size::TextSize;

    macro_rules! check_conversion {
        ($line_index:ident : $wide_line_col:expr => $text_size:expr ) => {
            let encoding = WideEncoding::Utf16;

            let line_col = $line_index.to_utf8(encoding, $wide_line_col);
            let offset = $line_index.offset(line_col);
            assert_eq!(offset, Some($text_size));

            let line_col = $line_index.line_col(offset.unwrap());
            let wide_line_col = $line_index.to_wide(encoding, line_col.unwrap());
            assert_eq!(wide_line_col, Some($wide_line_col));
        };
    }

    #[test]
    fn empty_string() {
        let line_index = LineIndex::new("");
        check_conversion!(line_index: WideLineCol { line: 0, col: 0 } => TextSize::from(0));
    }

    #[test]
    fn empty_line() {
        let line_index = LineIndex::new("\n\n");
        check_conversion!(line_index: WideLineCol { line: 1, col: 0 } => TextSize::from(1));
    }

    #[test]
    fn line_end() {
        let line_index = LineIndex::new("abc\ndef\nghi");
        check_conversion!(line_index: WideLineCol { line: 1, col: 3 } => TextSize::from(7));
    }

    #[test]
    fn out_of_bounds_line() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.offset(LineCol { line: 5, col: 0 });
        assert!(offset.is_none());
    }

    #[test]
    fn unicode() {
        let line_index = LineIndex::new("'Jan 1, 2018 – Jan 1, 2019'");

        check_conversion!(line_index: WideLineCol { line: 0, col: 0 } => TextSize::from(0));
        check_conversion!(line_index: WideLineCol { line: 0, col: 1 } => TextSize::from(1));
        check_conversion!(line_index: WideLineCol { line: 0, col: 12 } => TextSize::from(12));
        check_conversion!(line_index: WideLineCol { line: 0, col: 13 } => TextSize::from(15));
        check_conversion!(line_index: WideLineCol { line: 0, col: 14 } => TextSize::from(18));
        check_conversion!(line_index: WideLineCol { line: 0, col: 15 } => TextSize::from(21));
        check_conversion!(line_index: WideLineCol { line: 0, col: 26 } => TextSize::from(32));
        check_conversion!(line_index: WideLineCol { line: 0, col: 27 } => TextSize::from(33));
    }

    #[ignore]
    #[test]
    fn test_every_chars() {
        let text: String = {
            let mut chars: Vec<char> = ((0 as char)..char::MAX).collect();
            chars.extend("\n".repeat(chars.len() / 16).chars());
            chars.into_iter().collect()
        };

        let line_index = LineIndex::new(&text);

        let mut lin_col = LineCol { line: 0, col: 0 };
        let mut col_utf16 = 0;
        let mut col_utf32 = 0;
        for (offset, char) in text.char_indices() {
            let got_offset = line_index.offset(lin_col).unwrap();
            assert_eq!(usize::from(got_offset), offset);

            let got_lin_col = line_index.line_col(got_offset).unwrap();
            assert_eq!(got_lin_col, lin_col);

            for enc in [Utf16, Utf32] {
                let wide_lin_col = line_index.to_wide(enc, lin_col).unwrap();
                let got_lin_col = line_index.to_utf8(enc, wide_lin_col);
                assert_eq!(got_lin_col, lin_col);

                let want_col = match enc {
                    Utf16 => col_utf16,
                    Utf32 => col_utf32,
                };
                assert_eq!(wide_lin_col.col, want_col)
            }

            if char == '\n' {
                lin_col.line += 1;
                lin_col.col = 0;
                col_utf16 = 0;
                col_utf32 = 0;
            } else {
                lin_col.col += char.len_utf8() as u32;
                col_utf16 += char.len_utf16() as u32;
                col_utf32 += 1;
            }
        }
    }
}
