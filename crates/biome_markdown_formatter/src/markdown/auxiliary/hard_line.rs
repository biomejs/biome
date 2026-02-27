use crate::prelude::*;
use biome_markdown_syntax::MdHardLine;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHardLine;
impl FormatNodeRule<MdHardLine> for FormatMdHardLine {
    fn fmt_fields(&self, node: &MdHardLine, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
