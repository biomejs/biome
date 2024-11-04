//! The crate contains a set of converters to translate between `lsp-types` and `biome_rowan` (and vice versa) types.

use biome_rowan::TextSize;
use tower_lsp::lsp_types::{ClientCapabilities, PositionEncodingKind};

pub mod from_proto;
pub mod line_index;
pub mod to_proto;

pub fn negotiated_encoding(capabilities: &ClientCapabilities) -> PositionEncoding {
    let client_encodings = match &capabilities.general {
        Some(general) => general.position_encodings.as_deref().unwrap_or_default(),
        None => &[],
    };

    for enc in client_encodings {
        if enc == &PositionEncodingKind::UTF8 {
            return PositionEncoding::Utf8;
        } else if enc == &PositionEncodingKind::UTF32 {
            return PositionEncoding::Wide(WideEncoding::Utf32);
        }
        // NB: intentionally prefer just about anything else to utf-16.
    }

    PositionEncoding::Wide(WideEncoding::Utf16)
}

#[derive(Clone, Copy, Debug)]
pub enum PositionEncoding {
    Utf8,
    Wide(WideEncoding),
}

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
    use crate::from_proto::offset;
    use crate::line_index::LineIndex;
    use crate::to_proto::position;
    use crate::WideEncoding::{Utf16, Utf32};
    use crate::{LineCol, PositionEncoding, WideEncoding};
    use biome_rowan::TextSize;
    use tower_lsp::lsp_types::Position;

    macro_rules! check_conversion {
        ($line_index:ident : $position:expr => $text_size:expr ) => {
            let position_encoding = PositionEncoding::Wide(WideEncoding::Utf16);

            let offset = offset(&$line_index, $position, position_encoding).ok();
            assert_eq!(offset, Some($text_size));

            let position = position(&$line_index, offset.unwrap(), position_encoding).ok();

            assert_eq!(position, Some($position));
        };
    }

    #[test]
    fn empty_string() {
        let line_index = LineIndex::new("");
        check_conversion!(line_index: Position { line: 0, character: 0 } => TextSize::from(0));
    }

    #[test]
    fn empty_line() {
        let line_index = LineIndex::new("\n\n");
        check_conversion!(line_index: Position { line: 1, character: 0 } => TextSize::from(1));
    }

    #[test]
    fn line_end() {
        let line_index = LineIndex::new("abc\ndef\nghi");
        check_conversion!(line_index: Position { line: 1, character: 3 } => TextSize::from(7));
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

        check_conversion!(line_index: Position { line: 0, character: 0 } => TextSize::from(0));
        check_conversion!(line_index: Position { line: 0, character: 1 } => TextSize::from(1));
        check_conversion!(line_index: Position { line: 0, character: 12 } => TextSize::from(12));
        check_conversion!(line_index: Position { line: 0, character: 13 } => TextSize::from(15));
        check_conversion!(line_index: Position { line: 0, character: 14 } => TextSize::from(18));
        check_conversion!(line_index: Position { line: 0, character: 15 } => TextSize::from(21));
        check_conversion!(line_index: Position { line: 0, character: 26 } => TextSize::from(32));
        check_conversion!(line_index: Position { line: 0, character: 27 } => TextSize::from(33));
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
