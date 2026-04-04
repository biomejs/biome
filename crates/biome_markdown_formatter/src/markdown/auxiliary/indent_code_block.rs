use crate::prelude::*;
use biome_markdown_syntax::MdIndentCodeBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentCodeBlock;
impl FormatNodeRule<MdIndentCodeBlock> for FormatMdIndentCodeBlock {
    fn fmt_fields(&self, node: &MdIndentCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
