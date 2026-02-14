use crate::prelude::*;
use biome_markdown_syntax::MdInlineLink;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineLink;
impl FormatNodeRule<MdInlineLink> for FormatMdInlineLink {
    fn fmt_fields(&self, node: &MdInlineLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
