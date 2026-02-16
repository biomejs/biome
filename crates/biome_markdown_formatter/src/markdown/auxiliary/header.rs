use crate::prelude::*;
use biome_markdown_syntax::MdHeader;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHeader;
impl FormatNodeRule<MdHeader> for FormatMdHeader {
    fn fmt_fields(&self, node: &MdHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
