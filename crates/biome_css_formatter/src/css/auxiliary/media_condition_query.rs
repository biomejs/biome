use crate::prelude::*;
use biome_css_syntax::CssMediaConditionQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaConditionQuery;
impl FormatNodeRule<CssMediaConditionQuery> for FormatCssMediaConditionQuery {
    fn fmt_fields(&self, node: &CssMediaConditionQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
