use crate::prelude::*;
use biome_css_syntax::{CssUrlValueRaw, CssUrlValueRawFields, is_css_whitespace_byte};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlValueRaw;
impl FormatNodeRule<CssUrlValueRaw> for FormatCssUrlValueRaw {
    fn fmt_fields(&self, node: &CssUrlValueRaw, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlValueRawFields { value_token } = node.as_fields();
        let value_token = value_token?;
        let token_text = value_token.text_trimmed();
        let trimmed =
            token_text.trim_end_matches(|c: char| c.is_ascii() && is_css_whitespace_byte(c as u8));
        let whitespace = &token_text[trimmed.len()..];

        // Keep escaped whitespace so `url(@a\ )` doesn't become the unclosed `url(@a\)`.
        let keep_escaped_whitespace = if whitespace.starts_with([' ', '\t']) {
            let backslashes = trimmed.len() - trimmed.trim_end_matches('\\').len();
            backslashes % 2 == 1
        } else {
            false
        };
        let trimmed = if keep_escaped_whitespace {
            &token_text[..trimmed.len() + 1]
        } else {
            trimmed
        };
        write!(
            f,
            [format_replaced(
                &value_token,
                &text(trimmed, Some(value_token.text_trimmed_range().start()))
            )]
        )
    }
}
