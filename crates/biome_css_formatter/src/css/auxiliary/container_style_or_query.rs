use crate::prelude::*;
use biome_css_syntax::CssContainerStyleOrQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleOrQuery;
impl FormatNodeRule<CssContainerStyleOrQuery> for FormatCssContainerStyleOrQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
