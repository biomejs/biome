//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::MaybeCurlyGritPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMaybeCurlyGritPattern;
impl FormatRule<MaybeCurlyGritPattern> for FormatMaybeCurlyGritPattern {
    type Context = GritFormatContext;
    fn fmt(&self, node: &MaybeCurlyGritPattern, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            MaybeCurlyGritPattern::AnyGritPattern(node) => node.format().fmt(f),
            MaybeCurlyGritPattern::CurlyGritPattern(node) => node.format().fmt(f),
        }
    }
}
