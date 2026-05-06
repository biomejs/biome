use crate::prelude::*;
use biome_markdown_syntax::MdThematicBreakBlock;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        node.parts().format().fmt(f)
    }
}
