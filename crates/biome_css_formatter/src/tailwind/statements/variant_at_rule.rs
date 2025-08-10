use crate::prelude::*;
use biome_css_syntax::TwVariantAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwVariantAtRule;
impl FormatNodeRule<TwVariantAtRule> for FormatTwVariantAtRule {
    fn fmt_fields(&self, node: &TwVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
