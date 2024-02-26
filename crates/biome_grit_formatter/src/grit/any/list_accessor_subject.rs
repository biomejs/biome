//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritListAccessorSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListAccessorSubject;
impl FormatRule<GritListAccessorSubject> for FormatGritListAccessorSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritListAccessorSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritListAccessorSubject::GritList(node) => node.format().fmt(f),
            GritListAccessorSubject::AnyGritContainer(node) => node.format().fmt(f),
        }
    }
}
