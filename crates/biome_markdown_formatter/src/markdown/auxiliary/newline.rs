use crate::prelude::*;
use biome_markdown_syntax::MdNewline;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdNewline;
impl FormatNodeRule<MdNewline> for FormatMdNewline {
    fn fmt_fields(&self, node: &MdNewline, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
