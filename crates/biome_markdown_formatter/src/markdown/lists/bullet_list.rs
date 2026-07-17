use crate::prelude::*;
use biome_markdown_syntax::MdBulletList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletList;
impl FormatRule<MdBulletList> for FormatMdBulletList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, _node: &MdBulletList, _f: &mut MarkdownFormatter) -> FormatResult<()> {
        debug_assert!(
            false,
            "This node should be formatted via FmtAnyList. Match AnyMdBlock and use as_any_list_item."
        );

        Ok(())
    }
}
