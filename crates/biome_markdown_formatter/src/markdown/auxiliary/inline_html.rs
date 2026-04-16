use crate::prelude::*;
use biome_markdown_syntax::MdInlineHtml;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineHtml;
impl FormatNodeRule<MdInlineHtml> for FormatMdInlineHtml {
    fn fmt_fields(&self, node: &MdInlineHtml, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.value().format().fmt(f)
    }
}
