use crate::prelude::*;
use biome_css_syntax::CssVariantAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssVariantAtRule;
impl FormatNodeRule<CssVariantAtRule> for FormatCssVariantAtRule {
    fn fmt_fields(&self, node: &CssVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
