use crate::cst::FormatMarkdownSyntaxNode;
use crate::{FormatMdSyntaxToken, MdFormatContext};
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

impl<'a> Format<MdFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<MdFormatContext>) -> FormatResult<()> {
        FormatMdSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a MarkdownSyntaxToken,
    content: Argument<'a, MdFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a MarkdownSyntaxToken,
    content: &'a impl Format<MdFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<MdFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<MdFormatContext>) -> FormatResult<()> {
        FormatMarkdownSyntaxNode.format_replaced(self.token, &self.content, f)
    }
}
