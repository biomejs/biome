//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritListAccessorSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritListAccessorSubject;
impl FormatRule<AnyGritListAccessorSubject> for FormatAnyGritListAccessorSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritListAccessorSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritListAccessorSubject::AnyGritContainer(node) => node.format().fmt(f),
            AnyGritListAccessorSubject::GritList(node) => node.format().fmt(f),
        }
    }
}
