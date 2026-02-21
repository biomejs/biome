use crate::prelude::*;
use biome_markdown_syntax::MdOrderedListItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderedListItem;
impl FormatNodeRule<MdOrderedListItem> for FormatMdOrderedListItem {
    fn fmt_fields(&self, node: &MdOrderedListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
