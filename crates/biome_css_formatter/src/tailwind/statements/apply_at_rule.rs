use crate::prelude::*;
use biome_css_syntax::TwApplyAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyAtRule;
impl FormatNodeRule<TwApplyAtRule> for FormatTwApplyAtRule {
    fn fmt_fields(&self, node: &TwApplyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
