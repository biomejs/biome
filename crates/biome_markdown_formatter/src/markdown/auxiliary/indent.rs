use crate::prelude::*;
use biome_markdown_syntax::MdIndent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndent;
impl FormatNodeRule<MdIndent> for FormatMdIndent {
    fn fmt_fields(&self, node: &MdIndent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
