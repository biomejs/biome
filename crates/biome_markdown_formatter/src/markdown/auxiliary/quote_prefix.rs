use crate::prelude::*;
use biome_markdown_syntax::MdQuotePrefix;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuotePrefix;
impl FormatNodeRule<MdQuotePrefix> for FormatMdQuotePrefix {
    fn fmt_fields(&self, node: &MdQuotePrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
