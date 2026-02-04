use crate::prelude::*;
use biome_markdown_syntax::MdIndentedCodeLine;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentedCodeLine;
impl FormatNodeRule<MdIndentedCodeLine> for FormatMdIndentedCodeLine {
    fn fmt_fields(&self, node: &MdIndentedCodeLine, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
