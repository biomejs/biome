//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSyntaxComponent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSyntaxComponent;
impl FormatRule<AnyCssSyntaxComponent> for FormatAnyCssSyntaxComponent {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSyntaxComponent, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSyntaxComponent::CssSyntaxComponent(node) => node.format().fmt(f),
            AnyCssSyntaxComponent::CssSyntaxComponentWithoutMultiplier(node) => {
                node.format().fmt(f)
            }
        }
    }
}
