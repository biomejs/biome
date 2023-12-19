use crate::prelude::*;
use biome_css_syntax::CssMediaTypeQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaTypeQuery;
impl FormatNodeRule<CssMediaTypeQuery> for FormatCssMediaTypeQuery {
    fn fmt_fields(&self, node: &CssMediaTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
