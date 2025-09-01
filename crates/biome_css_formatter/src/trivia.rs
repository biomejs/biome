use crate::prelude::CssFormatContext;
use crate::{CssFormatter, FormatCssSyntaxToken};
use biome_css_syntax::CssSyntaxToken;
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{
    NumberFormatOptions, format_trimmed_number, syntax_token_cow_slice,
};
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};

pub(crate) struct FormatRemoved<'a> {
    token: &'a CssSyntaxToken,
}

pub(crate) fn format_removed(token: &CssSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<CssFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        FormatCssSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a CssSyntaxToken,
    content: Argument<'a, CssFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a CssSyntaxToken,
    content: &'a impl Format<CssFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<CssFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        FormatCssSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}

pub fn format_number_token(
    token: &CssSyntaxToken,
    options: NumberFormatOptions,
) -> CleanedNumberLiteralText<'_> {
    CleanedNumberLiteralText { token, options }
}

pub(crate) struct CleanedNumberLiteralText<'a> {
    token: &'a CssSyntaxToken,
    options: NumberFormatOptions,
}

impl Format<CssFormatContext> for CleanedNumberLiteralText<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
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

pub(crate) fn on_skipped(token: &CssSyntaxToken, f: &mut CssFormatter) -> FormatResult<()> {
    FormatCssSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &CssSyntaxToken, f: &mut CssFormatter) -> FormatResult<()> {
    FormatCssSyntaxToken.format_removed(token, f)
}
