use crate::prelude::*;
use biome_markdown_syntax::MdInlineImageAlt;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineImageAlt;
impl FormatNodeRule<MdInlineImageAlt> for FormatMdInlineImageAlt {
    fn fmt_fields(&self, node: &MdInlineImageAlt, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
