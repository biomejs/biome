use crate::prelude::*;
use biome_markdown_syntax::MdBlockList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBlockList;
impl FormatRule<MdBlockList> for FormatMdBlockList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdBlockList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
