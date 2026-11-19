use crate::prelude::*;
use biome_markdown_syntax::MdxEsmBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxEsmBlock;
impl FormatNodeRule<MdxEsmBlock> for FormatMdxEsmBlock {
    fn fmt_fields(&self, node: &MdxEsmBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
