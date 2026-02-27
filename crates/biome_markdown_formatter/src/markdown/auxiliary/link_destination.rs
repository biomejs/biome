use crate::prelude::*;
use biome_markdown_syntax::MdLinkDestination;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkDestination;
impl FormatNodeRule<MdLinkDestination> for FormatMdLinkDestination {
    fn fmt_fields(&self, node: &MdLinkDestination, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
