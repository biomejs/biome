use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHardLine, MdTextual};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHardLine;

impl FormatNodeRule<MdHardLine> for FormatMdHardLine {
    fn fmt_fields(&self, node: &MdHardLine, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let text_content = token.text();

        if is_followed_by_empty_line(node) {
            return write!(f, [format_removed(&token), hard_line_break()]);
        }

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
            // Given two or more spaces in MdHardLine, only two spaces has semantic meaning
            // so we are adding back two spaces as required by the spec
            // https://spec.commonmark.org/0.31.2/#hard-line-break
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

fn is_followed_by_empty_line(node: &MdHardLine) -> bool {
    node.syntax()
        .next_sibling()
        .and_then(MdTextual::cast)
        .is_some_and(|text| text.is_newline().unwrap_or(false))
}
