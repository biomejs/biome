use crate::prelude::*;
use biome_markdown_syntax::MdIndentTokenList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentTokenList;
impl FormatRule<MdIndentTokenList> for FormatMdIndentTokenList {
    type Context = MdFormatContext;
    fn fmt(&self, node: &MdIndentTokenList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
