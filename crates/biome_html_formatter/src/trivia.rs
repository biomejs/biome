use crate::FormatHtmlSyntaxToken;
use crate::prelude::HtmlFormatContext;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_html_syntax::HtmlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a HtmlSyntaxToken,
}

pub(crate) fn format_removed(token: &HtmlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<HtmlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        FormatHtmlSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a HtmlSyntaxToken,
    content: Argument<'a, HtmlFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a HtmlSyntaxToken,
    content: &'a impl Format<HtmlFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<HtmlFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        FormatHtmlSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}
