use crate::prelude::*;
use biome_markdown_syntax::MdQuoteIndent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuoteIndent;
impl FormatNodeRule<MdQuoteIndent> for FormatMdQuoteIndent {
    fn fmt_fields(&self, node: &MdQuoteIndent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
