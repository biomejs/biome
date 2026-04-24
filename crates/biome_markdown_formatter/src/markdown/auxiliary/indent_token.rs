use crate::prelude::*;
use biome_markdown_syntax::MdIndentToken;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentToken;
impl FormatNodeRule<MdIndentToken> for FormatMdIndentToken {
    fn fmt_fields(&self, node: &MdIndentToken, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.md_indent_char_token().format().fmt(f)
    }
}
