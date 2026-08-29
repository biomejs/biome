use crate::prelude::*;
use biome_markdown_syntax::GfmStrikethrough;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmStrikethrough;
impl FormatNodeRule<GfmStrikethrough> for FormatGfmStrikethrough {
    fn fmt_fields(&self, node: &GfmStrikethrough, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
