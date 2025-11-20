use crate::prelude::JsonFormatContext;
use crate::{FormatJsonSyntaxToken, JsonFormatter};
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{
    NumberFormatOptions, format_trimmed_number, syntax_token_cow_slice,
};
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_json_syntax::JsonSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a JsonSyntaxToken,
}

pub(crate) fn format_removed(token: &JsonSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<JsonFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<JsonFormatContext>) -> FormatResult<()> {
        FormatJsonSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a JsonSyntaxToken,
    content: Argument<'a, JsonFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a JsonSyntaxToken,
    content: &'a impl Format<JsonFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<JsonFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<JsonFormatContext>) -> FormatResult<()> {
        FormatJsonSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}

pub fn format_number_token(
    token: &JsonSyntaxToken,
    options: NumberFormatOptions,
) -> CleanedNumberLiteralText<'_> {
    CleanedNumberLiteralText { token, options }
}

pub(crate) struct CleanedNumberLiteralText<'a> {
    token: &'a JsonSyntaxToken,
    options: NumberFormatOptions,
}

impl Format<JsonFormatContext> for CleanedNumberLiteralText<'_> {
    fn fmt(&self, f: &mut JsonFormatter) -> FormatResult<()> {
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

pub(crate) fn on_skipped(token: &JsonSyntaxToken, f: &mut JsonFormatter) -> FormatResult<()> {
    FormatJsonSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &JsonSyntaxToken, f: &mut JsonFormatter) -> FormatResult<()> {
    FormatJsonSyntaxToken.format_removed(token, f)
}
