use crate::prelude::*;
use biome_markdown_syntax::MdInlineItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItemList;
impl FormatRule<MdInlineItemList> for FormatMdInlineItemList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
