use crate::PositionEncoding;
use anyhow::{Context, Result};
use biome_line_index::{LineCol, LineIndex, WideLineCol};
use biome_text_size::{TextRange, TextSize};
use tower_lsp_server::lsp_types::{Position, Range};

/// The function is used to convert a LSP position to TextSize.
pub fn offset(
    line_index: &LineIndex,
    position: Position,
    position_encoding: PositionEncoding,
) -> Result<TextSize> {
    let line_col = match position_encoding {
        PositionEncoding::Utf8 => LineCol {
            line: position.line,
            col: position.character,
        },
        PositionEncoding::Wide(enc) => {
            let line_col = WideLineCol {
                line: position.line,
                col: position.character,
            };
            line_index.to_utf8(enc, line_col)
        }
    };

    line_index
        .offset(line_col)
        .with_context(|| format!("position {position:?} is out of range"))
}

/// The function is used to convert a LSP range to TextRange.
pub fn text_range(
    line_index: &LineIndex,
    range: Range,
    position_encoding: PositionEncoding,
) -> Result<TextRange> {
    let start = offset(line_index, range.start, position_encoding)?;
    let end = offset(line_index, range.end, position_encoding)?;
    Ok(TextRange::new(start, end))
}
