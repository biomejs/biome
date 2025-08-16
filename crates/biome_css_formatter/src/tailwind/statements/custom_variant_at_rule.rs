use crate::prelude::*;
use biome_css_syntax::TwCustomVariantAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwCustomVariantAtRule;
impl FormatNodeRule<TwCustomVariantAtRule> for FormatTwCustomVariantAtRule {
    fn fmt_fields(&self, node: &TwCustomVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
