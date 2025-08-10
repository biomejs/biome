//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssTwCustomVariantSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssTwCustomVariantSelector;
impl FormatRule<AnyCssTwCustomVariantSelector> for FormatAnyCssTwCustomVariantSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssTwCustomVariantSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssTwCustomVariantSelector::AnyCssRuleBlock(node) => node.format().fmt(f),
            AnyCssTwCustomVariantSelector::CssTwCustomVariantShorthand(node) => {
                node.format().fmt(f)
            }
        }
    }
}
