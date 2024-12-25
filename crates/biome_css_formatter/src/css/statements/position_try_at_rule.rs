use crate::prelude::*;
use biome_css_syntax::CssPositionTryAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPositionTryAtRule;
impl FormatNodeRule<CssPositionTryAtRule> for FormatCssPositionTryAtRule {
    fn fmt_fields(&self, node: &CssPositionTryAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
