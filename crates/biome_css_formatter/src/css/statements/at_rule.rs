use crate::prelude::*;
use biome_css_syntax::CssAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAtRule;
impl FormatNodeRule<CssAtRule> for FormatCssAtRule {
    fn fmt_fields(&self, node: &CssAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
