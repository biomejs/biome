//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritContainer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritContainer;
impl FormatRule<AnyGritContainer> for FormatAnyGritContainer {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritContainer, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritContainer::GritBogusContainer(node) => node.format().fmt(f),
            AnyGritContainer::GritListAccessor(node) => node.format().fmt(f),
            AnyGritContainer::GritMapAccessor(node) => node.format().fmt(f),
            AnyGritContainer::GritVariable(node) => node.format().fmt(f),
        }
    }
}
