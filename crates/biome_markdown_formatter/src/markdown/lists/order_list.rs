use crate::prelude::*;
use biome_markdown_syntax::MdOrderList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdOrderList;
impl FormatRule<MdOrderList> for FormatMdOrderList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdOrderList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
