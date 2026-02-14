use crate::prelude::*;
use biome_markdown_syntax::MdQuote;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuote;
impl FormatNodeRule<MdQuote> for FormatMdQuote {
    fn fmt_fields(&self, node: &MdQuote, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
