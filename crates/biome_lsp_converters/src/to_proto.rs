use crate::line_index::LineIndex;
use crate::PositionEncoding;
use anyhow::{Context, Result};
use biome_rowan::{TextRange, TextSize};
use tower_lsp::lsp_types;

/// The function is used to convert TextSize to a LSP position.
pub fn position(
    line_index: &LineIndex,
    offset: TextSize,
    position_encoding: PositionEncoding,
) -> Result<lsp_types::Position> {
    let line_col = line_index
        .line_col(offset)
        .with_context(|| format!("could not convert offset {offset:?} into a line-column index"))?;

    let position = match position_encoding {
        PositionEncoding::Utf8 => lsp_types::Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index
                .to_wide(enc, line_col)
                .with_context(|| format!("could not convert {line_col:?} into wide line column"))?;
            lsp_types::Position::new(line_col.line, line_col.col)
        }
    };

    Ok(position)
}

/// The function is used to convert TextRange to a LSP range.
pub fn range(
    line_index: &LineIndex,
    range: TextRange,
    position_encoding: PositionEncoding,
) -> Result<lsp_types::Range> {
    let start = position(line_index, range.start(), position_encoding)?;
    let end = position(line_index, range.end(), position_encoding)?;
    Ok(lsp_types::Range::new(start, end))
}
