use crate::prelude::*;
use biome_markdown_syntax::{MdOrderedListItem, MdOrderedListItemFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderedListItem;
impl FormatNodeRule<MdOrderedListItem> for FormatMdOrderedListItem {
    fn fmt_fields(&self, node: &MdOrderedListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdOrderedListItemFields { md_bullet_list } = node.as_fields();

        f.join().entries(md_bullet_list.iter().formatted()).finish()
    }
}
