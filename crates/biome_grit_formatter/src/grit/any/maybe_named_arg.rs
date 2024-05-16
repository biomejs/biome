//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritMaybeNamedArg;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritMaybeNamedArg;
impl FormatRule<AnyGritMaybeNamedArg> for FormatAnyGritMaybeNamedArg {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritMaybeNamedArg, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritMaybeNamedArg::AnyGritPattern(node) => node.format().fmt(f),
            AnyGritMaybeNamedArg::GritBogusNamedArg(node) => node.format().fmt(f),
            AnyGritMaybeNamedArg::GritNamedArg(node) => node.format().fmt(f),
        }
    }
}
