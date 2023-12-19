use crate::prelude::*;
use biome_css_syntax::CssMediaQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQuery;
impl FormatNodeRule<CssMediaQuery> for FormatCssMediaQuery {
    fn fmt_fields(&self, node: &CssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
