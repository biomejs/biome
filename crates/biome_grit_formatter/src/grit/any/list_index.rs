//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritListIndex;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritListIndex;
impl FormatRule<AnyGritListIndex> for FormatAnyGritListIndex {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritListIndex, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritListIndex::AnyGritContainer(node) => node.format().fmt(f),
            AnyGritListIndex::GritIntLiteral(node) => node.format().fmt(f),
            AnyGritListIndex::GritNegativeIntLiteral(node) => node.format().fmt(f),
        }
    }
}
