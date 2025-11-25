use crate::prelude::*;
use biome_css_syntax::CssReturnsStatement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssReturnsStatement;
impl FormatNodeRule<CssReturnsStatement> for FormatCssReturnsStatement {
    fn fmt_fields(&self, node: &CssReturnsStatement, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
