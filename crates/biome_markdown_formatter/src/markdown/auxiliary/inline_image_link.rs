use crate::prelude::*;
use biome_markdown_syntax::MdInlineImageLink;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineImageLink;
impl FormatNodeRule<MdInlineImageLink> for FormatMdInlineImageLink {
    fn fmt_fields(&self, node: &MdInlineImageLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
