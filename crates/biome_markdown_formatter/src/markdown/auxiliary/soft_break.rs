use crate::prelude::*;
use biome_markdown_syntax::MdSoftBreak;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdSoftBreak;
impl FormatNodeRule<MdSoftBreak> for FormatMdSoftBreak {
    fn fmt_fields(&self, node: &MdSoftBreak, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
