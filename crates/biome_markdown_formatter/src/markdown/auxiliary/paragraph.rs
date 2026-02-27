use crate::prelude::*;
use biome_markdown_syntax::MdParagraph;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdParagraph;
impl FormatNodeRule<MdParagraph> for FormatMdParagraph {
    fn fmt_fields(&self, node: &MdParagraph, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
