//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritMapAccessorSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapAccessorSubject;
impl FormatRule<GritMapAccessorSubject> for FormatGritMapAccessorSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritMapAccessorSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritMapAccessorSubject::GritMap(node) => node.format().fmt(f),
            GritMapAccessorSubject::AnyGritContainer(node) => node.format().fmt(f),
        }
    }
}
