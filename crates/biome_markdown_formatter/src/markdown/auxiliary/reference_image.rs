use crate::prelude::*;
use biome_markdown_syntax::MdReferenceImage;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceImage;
impl FormatNodeRule<MdReferenceImage> for FormatMdReferenceImage {
    fn fmt_fields(&self, node: &MdReferenceImage, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
