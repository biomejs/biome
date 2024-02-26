//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritNamedArg;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritNamedArg;
impl FormatRule<AnyGritNamedArg> for FormatAnyGritNamedArg {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritNamedArg, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritNamedArg::GritNamedArg(node) => node.format().fmt(f),
            AnyGritNamedArg::GritNamedArgWithDefault(node) => node.format().fmt(f),
            AnyGritNamedArg::GritBogusNamedArg(node) => node.format().fmt(f),
        }
    }
}
