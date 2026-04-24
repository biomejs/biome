use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdBulletListItem, MdBulletListItemFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletListItem;
impl FormatNodeRule<MdBulletListItem> for FormatMdBulletListItem {
    fn fmt_fields(&self, node: &MdBulletListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdBulletListItemFields { md_bullet_list } = node.as_fields();
        write!(f, [md_bullet_list.format()])
    }
}
