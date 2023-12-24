//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSubSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSubSelector;
impl FormatRule<AnyCssSubSelector> for FormatAnyCssSubSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSubSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSubSelector::CssIdSelector(node) => node.format().fmt(f),
            AnyCssSubSelector::CssClassSelector(node) => node.format().fmt(f),
            AnyCssSubSelector::CssAttributeSelector(node) => node.format().fmt(f),
            AnyCssSubSelector::CssPseudoClassSelector(node) => node.format().fmt(f),
            AnyCssSubSelector::CssPseudoElementSelector(node) => node.format().fmt(f),
            AnyCssSubSelector::CssBogusSubSelector(node) => node.format().fmt(f),
        }
    }
}
