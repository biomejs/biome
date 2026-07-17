use crate::prelude::*;
use biome_markdown_syntax::MdBulletListItem;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletListItem;
impl FormatNodeRule<MdBulletListItem> for FormatMdBulletListItem {
    fn fmt_fields(&self, _node: &MdBulletListItem, _f: &mut MarkdownFormatter) -> FormatResult<()> {
        debug_assert!(
            false,
            "This node should be formatted via FmtAnyList. Match AnyMdBlock and use as_any_list_item."
        );

        Ok(())
    }
}
