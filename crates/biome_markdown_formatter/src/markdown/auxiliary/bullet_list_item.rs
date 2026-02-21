use crate::prelude::*;
use biome_markdown_syntax::MdBulletListItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletListItem;
impl FormatNodeRule<MdBulletListItem> for FormatMdBulletListItem {
    fn fmt_fields(&self, node: &MdBulletListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
