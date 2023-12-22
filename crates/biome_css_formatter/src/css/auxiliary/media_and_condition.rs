use crate::prelude::*;
use biome_css_syntax::CssMediaAndCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAndCondition;
impl FormatNodeRule<CssMediaAndCondition> for FormatCssMediaAndCondition {
    fn fmt_fields(&self, node: &CssMediaAndCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
