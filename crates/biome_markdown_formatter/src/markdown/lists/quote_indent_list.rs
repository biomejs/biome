use crate::prelude::*;
use biome_markdown_syntax::MdQuoteIndentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuoteIndentList;
impl FormatRule<MdQuoteIndentList> for FormatMdQuoteIndentList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdQuoteIndentList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
