//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwCustomVariantSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwCustomVariantSelector;
impl FormatRule<AnyTwCustomVariantSelector> for FormatAnyTwCustomVariantSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwCustomVariantSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwCustomVariantSelector::AnyCssRuleBlock(node) => node.format().fmt(f),
            AnyTwCustomVariantSelector::TwCustomVariantShorthand(node) => node.format().fmt(f),
        }
    }
}
