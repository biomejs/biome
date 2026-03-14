use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::MdHardLine;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHardLine;
impl FormatNodeRule<MdHardLine> for FormatMdHardLine {
    fn fmt_fields(&self, node: &MdHardLine, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let text_content = token.text();

        if text_content.trim_end().ends_with('\\') {
            // Preserve backslash form
            write!(
                f,
                [
                    format_removed(&token),
                    text("\\", token.text_range().start()),
                    hard_line_break()
                ]
            )
        } else {
            // Normalize spaces to 2 spaces
            write!(
                f,
                [
                    format_removed(&token),
                    text("  ", token.text_range().start()),
                    hard_line_break(),
                ]
            )
        }
    }
}
