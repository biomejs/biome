use crate::prelude::*;
use biome_markdown_syntax::MdReferenceLink;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceLink;
impl FormatNodeRule<MdReferenceLink> for FormatMdReferenceLink {
    fn fmt_fields(&self, node: &MdReferenceLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
