use crate::prelude::*;
use biome_css_syntax::TwConfigAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwConfigAtRule;
impl FormatNodeRule<TwConfigAtRule> for FormatTwConfigAtRule {
    fn fmt_fields(&self, node: &TwConfigAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
