use crate::prelude::*;
use biome_css_syntax::CssContainerStyleNotQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleNotQuery;
impl FormatNodeRule<CssContainerStyleNotQuery> for FormatCssContainerStyleNotQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
