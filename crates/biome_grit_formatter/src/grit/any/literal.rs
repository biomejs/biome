//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritLiteral;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritLiteral;
impl FormatRule<AnyGritLiteral> for FormatAnyGritLiteral {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritLiteral::GritBogusLiteral(node) => node.format().fmt(f),
            AnyGritLiteral::GritBooleanLiteral(node) => node.format().fmt(f),
            AnyGritLiteral::GritCodeSnippet(node) => node.format().fmt(f),
            AnyGritLiteral::GritDoubleLiteral(node) => node.format().fmt(f),
            AnyGritLiteral::GritIntLiteral(node) => node.format().fmt(f),
            AnyGritLiteral::GritList(node) => node.format().fmt(f),
            AnyGritLiteral::GritMap(node) => node.format().fmt(f),
            AnyGritLiteral::GritStringLiteral(node) => node.format().fmt(f),
            AnyGritLiteral::GritUndefinedLiteral(node) => node.format().fmt(f),
        }
    }
}
