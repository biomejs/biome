use crate::prelude::*;
use biome_css_syntax::CssLayerAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerAtRule;
impl FormatNodeRule<CssLayerAtRule> for FormatCssLayerAtRule {
    fn fmt_fields(&self, node: &CssLayerAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
