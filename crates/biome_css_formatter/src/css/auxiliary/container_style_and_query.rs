use crate::prelude::*;
use biome_css_syntax::CssContainerStyleAndQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleAndQuery;
impl FormatNodeRule<CssContainerStyleAndQuery> for FormatCssContainerStyleAndQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleAndQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
