use crate::bullet_list::BulletListPrinter;
use crate::prelude::*;
use biome_markdown_syntax::{MdOrderedListItem, MdOrderedListItemFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderedListItem;
impl FormatNodeRule<MdOrderedListItem> for FormatMdOrderedListItem {
    fn fmt_fields(&self, node: &MdOrderedListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdOrderedListItemFields { md_bullet_list } = node.as_fields();
        let list_printer = BulletListPrinter::new(&md_bullet_list, true);
        list_printer.fmt(f)
    }
}
