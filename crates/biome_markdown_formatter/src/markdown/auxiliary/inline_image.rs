use crate::prelude::*;
use biome_markdown_syntax::MdInlineImage;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineImage;
impl FormatNodeRule<MdInlineImage> for FormatMdInlineImage {
    fn fmt_fields(&self, node: &MdInlineImage, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
