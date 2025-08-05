use crate::prelude::*;
use biome_css_syntax::CssApplyAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssApplyAtRule;
impl FormatNodeRule<CssApplyAtRule> for FormatCssApplyAtRule {
    fn fmt_fields(&self, node: &CssApplyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
