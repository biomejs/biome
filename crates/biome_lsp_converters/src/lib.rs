//! The crate contains a set of converters to translate between `lsp-types` and `biome_rowan` (and vice versa) types.

#![deny(clippy::use_self)]

use biome_line_index::WideEncoding;
use tower_lsp_server::lsp_types::{ClientCapabilities, PositionEncodingKind};

pub mod from_proto;
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

#[cfg(test)]
mod tests {
    use crate::PositionEncoding;
    use crate::from_proto::offset;
    use crate::to_proto::position;
    use biome_line_index::{LineIndex, WideEncoding::Utf16};
    use biome_text_size::TextSize;
    use tower_lsp_server::lsp_types::Position;

    macro_rules! check_conversion {
        ($line_index:ident : $position:expr => $text_size:expr ) => {
            let position_encoding = PositionEncoding::Wide(Utf16);

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
}
