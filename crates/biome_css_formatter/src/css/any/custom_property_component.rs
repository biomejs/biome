//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssCustomPropertyComponent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssCustomPropertyComponent;
impl FormatRule<AnyCssCustomPropertyComponent> for FormatAnyCssCustomPropertyComponent {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssCustomPropertyComponent, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssCustomPropertyComponent::AnyCssDimension(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomPropertyBracedBlock(node) => {
                node.format().fmt(f)
            }
            AnyCssCustomPropertyComponent::CssCustomPropertyBracketedBlock(node) => {
                node.format().fmt(f)
            }
            AnyCssCustomPropertyComponent::CssCustomPropertyDelimiter(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomPropertyFunction(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomPropertyParenthesizedBlock(node) => {
                node.format().fmt(f)
            }
            AnyCssCustomPropertyComponent::CssNumber(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssString(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::ScssInterpolatedString(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
