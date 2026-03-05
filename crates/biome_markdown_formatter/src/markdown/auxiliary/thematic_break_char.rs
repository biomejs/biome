use crate::prelude::*;
use biome_markdown_syntax::MdThematicBreakChar;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakChar;
impl FormatNodeRule<MdThematicBreakChar> for FormatMdThematicBreakChar {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakChar,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
