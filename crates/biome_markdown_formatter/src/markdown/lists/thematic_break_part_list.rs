use crate::prelude::*;
use biome_markdown_syntax::MdThematicBreakPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakPartList;
impl FormatRule<MdThematicBreakPartList> for FormatMdThematicBreakPartList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdThematicBreakPartList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
