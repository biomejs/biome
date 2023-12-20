use crate::prelude::*;
use biome_css_syntax::CssCounterStyleAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCounterStyleAtRule;
impl FormatNodeRule<CssCounterStyleAtRule> for FormatCssCounterStyleAtRule {
    fn fmt_fields(&self, node: &CssCounterStyleAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
