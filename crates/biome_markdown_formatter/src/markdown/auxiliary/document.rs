use crate::prelude::*;
use biome_markdown_syntax::MdDocument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdDocument;
impl FormatNodeRule<MdDocument> for FormatMdDocument {
    fn fmt_fields(&self, node: &MdDocument, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
