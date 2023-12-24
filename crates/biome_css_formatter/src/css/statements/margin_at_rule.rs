use crate::prelude::*;
use biome_css_syntax::CssMarginAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMarginAtRule;
impl FormatNodeRule<CssMarginAtRule> for FormatCssMarginAtRule {
    fn fmt_fields(&self, node: &CssMarginAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
