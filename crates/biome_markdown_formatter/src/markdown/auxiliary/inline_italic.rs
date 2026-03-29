use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineItalic, MdInlineItalicFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItalic;
impl FormatNodeRule<MdInlineItalic> for FormatMdInlineItalic {
    fn fmt_fields(&self, node: &MdInlineItalic, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineItalicFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        write!(f, [l_fence.format(), content.format(), r_fence.format()])
    }
}
