use crate::prelude::*;
use biome_markdown_syntax::MdOrderedListItem;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderedListItem;
impl FormatNodeRule<MdOrderedListItem> for FormatMdOrderedListItem {
    fn fmt_fields(
        &self,
        _node: &MdOrderedListItem,
        _f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        // let backtrace = Backtrace::force_capture();
        debug_assert!(
            false,
            "This node should be formatted via FmtAnyList. Match AnyMdBlock and use as_any_list_item.",
        );

        Ok(())
    }
}
