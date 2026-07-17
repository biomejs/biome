use crate::FormatYamlSyntaxToken;
use crate::prelude::{YamlFormatContext, YamlFormatter};
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_yaml_syntax::YamlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a YamlSyntaxToken,
}

pub(crate) fn format_removed(token: &YamlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<YamlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        FormatYamlSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a YamlSyntaxToken,
    content: Argument<'a, YamlFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a YamlSyntaxToken,
    content: &'a impl Format<YamlFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<YamlFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        FormatYamlSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}

pub(crate) fn on_skipped(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_removed(token, f)
}
