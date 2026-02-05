use crate::prelude::*;
use biome_markdown_syntax::MdIndentedCodeLineList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentedCodeLineList;
impl FormatRule<MdIndentedCodeLineList> for FormatMdIndentedCodeLineList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdIndentedCodeLineList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
