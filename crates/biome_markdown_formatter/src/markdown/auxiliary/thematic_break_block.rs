use crate::prelude::*;
use biome_markdown_syntax::MdThematicBreakBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
