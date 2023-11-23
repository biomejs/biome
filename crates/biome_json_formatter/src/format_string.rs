use crate::prelude::*;
use biome_formatter::token::string::{normalize_string, Quote};
use biome_json_syntax::JsonSyntaxToken;
use std::borrow::Cow;

pub(crate) fn format_string_token(token: &JsonSyntaxToken) -> CleanedStringLiteralText {
    CleanedStringLiteralText { token }
}

pub(crate) struct CleanedStringLiteralText<'token> {
    token: &'token JsonSyntaxToken,
}

impl Format<JsonFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<JsonFormatContext>) -> FormatResult<()> {
        let content = self.token.text_trimmed();
        let raw_content = &content[1..content.len() - 1];

        let text = match normalize_string(raw_content, Quote::Double, false) {
            Cow::Borrowed(_) => Cow::Borrowed(content),
            Cow::Owned(raw_content) => Cow::Owned(std::format!(
                "{}{}{}",
                Quote::Double.as_char(),
                raw_content,
                Quote::Double.as_char()
            )),
        };

        format_replaced(
            self.token,
            &syntax_token_cow_slice(text, self.token, self.token.text_trimmed_range().start()),
        )
        .fmt(f)
    }
}
