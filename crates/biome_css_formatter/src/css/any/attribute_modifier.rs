//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttributeModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttributeModifier;
impl FormatRule<AnyCssAttributeModifier> for FormatAnyCssAttributeModifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttributeModifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttributeModifier::CssAttributeModifier(node) => node.format().fmt(f),
            AnyCssAttributeModifier::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
