use crate::prelude::*;
use biome_markdown_syntax::MdOrderListItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderListItem;
impl FormatNodeRule<MdOrderListItem> for FormatMdOrderListItem {
    fn fmt_fields(&self, node: &MdOrderListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
