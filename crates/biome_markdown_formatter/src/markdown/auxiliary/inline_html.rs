use crate::prelude::*;
use biome_markdown_syntax::MdInlineHtml;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineHtml;
impl FormatNodeRule<MdInlineHtml> for FormatMdInlineHtml {
    fn fmt_fields(&self, node: &MdInlineHtml, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
