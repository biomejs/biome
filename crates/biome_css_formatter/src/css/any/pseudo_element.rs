//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPseudoElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoElement;
impl FormatRule<AnyCssPseudoElement> for FormatAnyCssPseudoElement {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPseudoElement, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPseudoElement::CssBogusPseudoElement(node) => node.format().fmt(f),
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(node) => node.format().fmt(f),
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(node) => node.format().fmt(f),
            AnyCssPseudoElement::CssPseudoElementIdentifier(node) => node.format().fmt(f),
        }
    }
}
