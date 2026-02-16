use crate::prelude::*;
use biome_markdown_syntax::MdHtmlBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHtmlBlock;
impl FormatNodeRule<MdHtmlBlock> for FormatMdHtmlBlock {
    fn fmt_fields(&self, node: &MdHtmlBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
