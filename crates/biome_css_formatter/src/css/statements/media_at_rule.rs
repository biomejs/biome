use crate::prelude::*;
use biome_css_syntax::CssMediaAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAtRule;
impl FormatNodeRule<CssMediaAtRule> for FormatCssMediaAtRule {
    fn fmt_fields(&self, node: &CssMediaAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
