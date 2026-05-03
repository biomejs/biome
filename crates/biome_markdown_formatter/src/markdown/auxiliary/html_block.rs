use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHtmlBlock, MdHtmlBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHtmlBlock;
impl FormatNodeRule<MdHtmlBlock> for FormatMdHtmlBlock {
    fn fmt_fields(&self, node: &MdHtmlBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdHtmlBlockFields { indent, content } = node.as_fields();

        write!(f, [indent.format(), content.format()])
    }
}
