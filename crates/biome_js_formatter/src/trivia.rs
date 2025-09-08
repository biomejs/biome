use crate::prelude::JsFormatContext;
use crate::{FormatJsSyntaxToken, JsFormatter};
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{
    NumberFormatOptions, format_trimmed_number, syntax_token_cow_slice,
};
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_js_syntax::JsSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a JsSyntaxToken,
}

pub(crate) fn format_removed(token: &JsSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<JsFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        FormatJsSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a JsSyntaxToken,
    content: Argument<'a, JsFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a JsSyntaxToken,
    content: &'a impl Format<JsFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<JsFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        FormatJsSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}

pub fn format_number_token(
    token: &JsSyntaxToken,
    options: NumberFormatOptions,
) -> CleanedNumberLiteralText<'_> {
    CleanedNumberLiteralText { token, options }
}

pub(crate) struct CleanedNumberLiteralText<'a> {
    token: &'a JsSyntaxToken,
    options: NumberFormatOptions,
}

impl Format<JsFormatContext> for CleanedNumberLiteralText<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                format_trimmed_number(self.token.text_trimmed(), self.options),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

pub(crate) fn on_skipped(token: &JsSyntaxToken, f: &mut JsFormatter) -> FormatResult<()> {
    FormatJsSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &JsSyntaxToken, f: &mut JsFormatter) -> FormatResult<()> {
    FormatJsSyntaxToken.format_removed(token, f)
}
