use crate::{FormatMdSyntaxToken, MdFormatContext};
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Format, FormatResult};
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
