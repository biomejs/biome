use crate::prelude::*;
use biome_markdown_syntax::MdHashList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHashList;
impl FormatRule<MdHashList> for FormatMdHashList {
    type Context = MdFormatContext;
    fn fmt(&self, node: &MdHashList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
