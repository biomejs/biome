//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSyntaxSingleComponent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSyntaxSingleComponent;
impl FormatRule<AnyCssSyntaxSingleComponent> for FormatAnyCssSyntaxSingleComponent {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSyntaxSingleComponent, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSyntaxSingleComponent::CssBogusSyntaxSingleComponent(node) => {
                node.format().fmt(f)
            }
            AnyCssSyntaxSingleComponent::CssIdentifier(node) => node.format().fmt(f),
            AnyCssSyntaxSingleComponent::CssSyntaxType(node) => node.format().fmt(f),
        }
    }
}
