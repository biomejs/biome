use crate::prelude::*;
use biome_markdown_syntax::MdIndentToken;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentToken;
impl FormatNodeRule<MdIndentToken> for FormatMdIndentToken {
    fn fmt_fields(&self, node: &MdIndentToken, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
