use crate::prelude::*;
use biome_css_syntax::CssSourceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSourceAtRule;
impl FormatNodeRule<CssSourceAtRule> for FormatCssSourceAtRule {
    fn fmt_fields(&self, node: &CssSourceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
