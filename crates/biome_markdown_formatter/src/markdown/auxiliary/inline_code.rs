use crate::prelude::*;
use biome_markdown_syntax::MdInlineCode;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineCode;
impl FormatNodeRule<MdInlineCode> for FormatMdInlineCode {
    fn fmt_fields(&self, node: &MdInlineCode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
