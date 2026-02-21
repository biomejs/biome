use crate::prelude::*;
use biome_markdown_syntax::MdLinkTitle;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkTitle;
impl FormatNodeRule<MdLinkTitle> for FormatMdLinkTitle {
    fn fmt_fields(&self, node: &MdLinkTitle, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
