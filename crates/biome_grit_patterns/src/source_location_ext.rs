use biome_diagnostics::display::{SourceFile, SourceLocation};
use biome_rowan::TextRange;
use grit_util::{Position, Range};

pub trait SourceFileExt {
    fn to_grit_range(&self, range: TextRange) -> Option<Range>;
}

impl SourceFileExt for SourceFile<'_> {
    fn to_grit_range(&self, range: TextRange) -> Option<Range> {
        let start = self.location(range.start()).ok()?;
        let end = self.location(range.end()).ok()?;

        Some(Range {
            start: start.to_grit_position(),
            end: end.to_grit_position(),
            start_byte: range.start().into(),
            end_byte: range.end().into(),
        })
    }
}

pub trait SourceLocationExt {
    fn to_grit_position(&self) -> Position;
}

impl SourceLocationExt for SourceLocation {
    fn to_grit_position(&self) -> Position {
        Position {
            column: self.column_number.get() as u32,
            line: self.line_number.get() as u32,
        }
    }
}
