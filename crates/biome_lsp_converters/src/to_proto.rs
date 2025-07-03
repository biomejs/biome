use crate::PositionEncoding;
use anyhow::{Context, Result};
use biome_line_index::LineIndex;
use biome_text_size::{TextRange, TextSize};
use tower_lsp_server::lsp_types::{Position, Range};

/// The function is used to convert TextSize to a LSP position.
pub fn position(
    line_index: &LineIndex,
    offset: TextSize,
    position_encoding: PositionEncoding,
) -> Result<Position> {
    let line_col = line_index
        .line_col(offset)
        .with_context(|| format!("could not convert offset {offset:?} into a line-column index"))?;

    let position = match position_encoding {
        PositionEncoding::Utf8 => Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index
                .to_wide(enc, line_col)
                .with_context(|| format!("could not convert {line_col:?} into wide line column"))?;
            Position::new(line_col.line, line_col.col)
        }
    };

    Ok(position)
}

/// The function is used to convert TextRange to a LSP range.
pub fn range(
    line_index: &LineIndex,
    range: TextRange,
    position_encoding: PositionEncoding,
) -> Result<Range> {
    let start = position(line_index, range.start(), position_encoding)?;
    let end = position(line_index, range.end(), position_encoding)?;
    Ok(Range::new(start, end))
}
