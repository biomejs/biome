use crate::MarkdownFormatContext;
use crate::cst::FormatMdSyntaxToken;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_markdown_syntax::MarkdownSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a MarkdownSyntaxToken,
}

pub(crate) fn format_removed(token: &MarkdownSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<MarkdownFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        FormatMdSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a MarkdownSyntaxToken,
    content: Argument<'a, MarkdownFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a MarkdownSyntaxToken,
    content: &'a impl Format<MarkdownFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<MarkdownFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        FormatMdSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}
