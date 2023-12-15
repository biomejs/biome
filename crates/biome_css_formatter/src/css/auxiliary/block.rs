use crate::prelude::*;
use biome_css_syntax::CssBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBlock;
impl FormatNodeRule<CssBlock> for FormatCssBlock {
    fn fmt_fields(&self, node: &CssBlock, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
