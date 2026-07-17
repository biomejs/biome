use crate::prelude::*;
use biome_markdown_syntax::MdQuoteIndent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuoteIndent;
impl FormatNodeRule<MdQuoteIndent> for FormatMdQuoteIndent {
    fn fmt_fields(&self, node: &MdQuoteIndent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.md_quote_pre_marker_indent_token().format().fmt(f)
    }
}
