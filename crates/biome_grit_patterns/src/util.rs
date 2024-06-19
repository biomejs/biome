use biome_rowan::TextRange;
use grit_util::{ByteRange, CodeRange};

pub trait TextRangeGritExt {
    fn to_byte_range(&self) -> ByteRange;

    fn to_code_range(&self, source: &str) -> CodeRange;
}

impl TextRangeGritExt for TextRange {
    fn to_byte_range(&self) -> ByteRange {
        ByteRange::new(self.start().into(), self.end().into())
    }

    fn to_code_range(&self, source: &str) -> CodeRange {
        CodeRange::new(self.start().into(), self.end().into(), source)
    }
}
