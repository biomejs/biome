//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritMapAccessorSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritMapAccessorSubject;
impl FormatRule<AnyGritMapAccessorSubject> for FormatAnyGritMapAccessorSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritMapAccessorSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritMapAccessorSubject::AnyGritContainer(node) => node.format().fmt(f),
            AnyGritMapAccessorSubject::GritMap(node) => node.format().fmt(f),
        }
    }
}
