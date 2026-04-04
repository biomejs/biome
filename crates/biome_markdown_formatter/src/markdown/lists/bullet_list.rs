use crate::prelude::*;
use biome_markdown_syntax::MdBulletList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletList;
impl FormatRule<MdBulletList> for FormatMdBulletList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdBulletList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
