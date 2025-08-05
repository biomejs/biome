use crate::prelude::*;
use biome_css_syntax::CssCustomVariantAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomVariantAtRule;
impl FormatNodeRule<CssCustomVariantAtRule> for FormatCssCustomVariantAtRule {
    fn fmt_fields(&self, node: &CssCustomVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
