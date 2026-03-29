use crate::prelude::*;
use crate::verbatim::format_verbatim_node;
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineCode, MdInlineCodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineCode;
impl FormatNodeRule<MdInlineCode> for FormatMdInlineCode {
    fn fmt_fields(&self, node: &MdInlineCode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineCodeFields {
            l_tick_token,
            content,
            r_tick_token,
        } = node.as_fields();

        // Code spans preserve their content verbatim (whitespace is significant)
        write!(
            f,
            [
                l_tick_token.format(),
                format_verbatim_node(content.syntax()),
                r_tick_token.format()
            ]
        )
    }
}
