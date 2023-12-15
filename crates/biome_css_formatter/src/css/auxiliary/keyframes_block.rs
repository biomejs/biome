use crate::prelude::*;
use biome_css_syntax::CssKeyframesBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesBlock;
impl FormatNodeRule<CssKeyframesBlock> for FormatCssKeyframesBlock {
    fn fmt_fields(&self, node: &CssKeyframesBlock, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
