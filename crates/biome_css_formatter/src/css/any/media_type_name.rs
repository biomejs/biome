//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaTypeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaTypeName;
impl FormatRule<AnyCssMediaTypeName> for FormatAnyCssMediaTypeName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaTypeName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaTypeName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssMediaTypeName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
