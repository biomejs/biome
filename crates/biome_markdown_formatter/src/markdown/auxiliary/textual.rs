use crate::prelude::*;
use biome_markdown_syntax::MdTextual;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdTextual;
impl FormatNodeRule<MdTextual> for FormatMdTextual {
    fn fmt_fields(&self, node: &MdTextual, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
