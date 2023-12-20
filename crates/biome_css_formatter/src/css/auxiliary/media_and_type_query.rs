use crate::prelude::*;
use biome_css_syntax::CssMediaAndTypeQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAndTypeQuery;
impl FormatNodeRule<CssMediaAndTypeQuery> for FormatCssMediaAndTypeQuery {
    fn fmt_fields(&self, node: &CssMediaAndTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
