use biome_rowan::TextRange;
use grit_util::ByteRange;

pub trait TextRangeGritExt {
    fn to_byte_range(&self) -> ByteRange;
}

impl TextRangeGritExt for TextRange {
    fn to_byte_range(&self) -> ByteRange {
        ByteRange::new(self.start().into(), self.end().into())
    }
}
