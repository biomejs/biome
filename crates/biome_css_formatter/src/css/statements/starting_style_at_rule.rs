use crate::prelude::*;
use biome_css_syntax::CssStartingStyleAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssStartingStyleAtRule;
impl FormatNodeRule<CssStartingStyleAtRule> for FormatCssStartingStyleAtRule {
    fn fmt_fields(&self, node: &CssStartingStyleAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
