use crate::prelude::*;
use biome_markdown_syntax::MdQuotePrefixList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuotePrefixList;
impl FormatRule<MdQuotePrefixList> for FormatMdQuotePrefixList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdQuotePrefixList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
