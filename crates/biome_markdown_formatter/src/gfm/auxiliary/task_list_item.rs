use crate::prelude::*;
use biome_markdown_syntax::GfmTaskListItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTaskListItem;
impl FormatNodeRule<GfmTaskListItem> for FormatGfmTaskListItem {
    fn fmt_fields(&self, node: &GfmTaskListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
