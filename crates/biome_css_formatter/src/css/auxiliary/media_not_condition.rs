use crate::prelude::*;
use biome_css_syntax::CssMediaNotCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaNotCondition;
impl FormatNodeRule<CssMediaNotCondition> for FormatCssMediaNotCondition {
    fn fmt_fields(&self, node: &CssMediaNotCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
