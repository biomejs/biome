use crate::prelude::*;
use biome_css_syntax::TwUtilityAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwUtilityAtRule;
impl FormatNodeRule<TwUtilityAtRule> for FormatTwUtilityAtRule {
    fn fmt_fields(&self, node: &TwUtilityAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
