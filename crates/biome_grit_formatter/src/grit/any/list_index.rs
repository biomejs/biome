//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritListIndex;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListIndex;
impl FormatRule<GritListIndex> for FormatGritListIndex {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritListIndex, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritListIndex::AnyGritContainer(node) => node.format().fmt(f),
            GritListIndex::GritNegativeIntLiteral(node) => node.format().fmt(f),
            GritListIndex::GritIntLiteral(node) => node.format().fmt(f),
        }
    }
}
