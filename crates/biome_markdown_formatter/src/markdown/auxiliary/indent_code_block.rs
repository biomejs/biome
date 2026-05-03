use crate::prelude::*;
use biome_markdown_syntax::{MdIndentCodeBlock, MdIndentCodeBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentCodeBlock;
impl FormatNodeRule<MdIndentCodeBlock> for FormatMdIndentCodeBlock {
    fn fmt_fields(&self, node: &MdIndentCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdIndentCodeBlockFields { content } = node.as_fields();

        content.format().fmt(f)
    }
}
