use crate::prelude::*;
use biome_css_syntax::CssMediaOrCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaOrCondition;
impl FormatNodeRule<CssMediaOrCondition> for FormatCssMediaOrCondition {
    fn fmt_fields(&self, node: &CssMediaOrCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
