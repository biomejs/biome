use crate::prelude::*;
use biome_css_syntax::CssContainerNotQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerNotQuery;
impl FormatNodeRule<CssContainerNotQuery> for FormatCssContainerNotQuery {
    fn fmt_fields(&self, node: &CssContainerNotQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
