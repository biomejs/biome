use crate::FormatYamlSyntaxToken;
use crate::prelude::YamlFormatContext;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Argument, Format, FormatResult};
use biome_yaml_syntax::YamlSyntaxToken;

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
