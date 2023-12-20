use crate::prelude::*;
use biome_css_syntax::CssContainerAndQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAndQuery;
impl FormatNodeRule<CssContainerAndQuery> for FormatCssContainerAndQuery {
    fn fmt_fields(&self, node: &CssContainerAndQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
