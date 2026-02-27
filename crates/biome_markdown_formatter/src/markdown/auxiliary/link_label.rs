use crate::prelude::*;
use biome_markdown_syntax::MdLinkLabel;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkLabel;
impl FormatNodeRule<MdLinkLabel> for FormatMdLinkLabel {
    fn fmt_fields(&self, node: &MdLinkLabel, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
