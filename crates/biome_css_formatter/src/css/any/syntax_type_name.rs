//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSyntaxTypeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSyntaxTypeName;
impl FormatRule<AnyCssSyntaxTypeName> for FormatAnyCssSyntaxTypeName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSyntaxTypeName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSyntaxTypeName::CssRegularSyntaxTypeName(node) => node.format().fmt(f),
            AnyCssSyntaxTypeName::CssUnknownSyntaxTypeName(node) => node.format().fmt(f),
        }
    }
}
