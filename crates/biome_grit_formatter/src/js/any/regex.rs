//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritRegex;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritRegex;
impl FormatRule<AnyGritRegex> for FormatAnyGritRegex {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritRegex, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritRegex::GritRegexLiteral(node) => node.format().fmt(f),
            AnyGritRegex::GritSnippetRegexLiteral(node) => node.format().fmt(f),
        }
    }
}
