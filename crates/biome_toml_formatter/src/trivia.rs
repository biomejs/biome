use crate::prelude::TomlFormatContext;
use crate::{FormatTomlSyntaxToken, TomlFormatter};
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Format, FormatResult};
use biome_toml_syntax::TomlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a TomlSyntaxToken,
}

pub(crate) fn format_removed(token: &TomlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<TomlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<TomlFormatContext>) -> FormatResult<()> {
        FormatTomlSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) fn on_skipped(token: &TomlSyntaxToken, f: &mut TomlFormatter) -> FormatResult<()> {
    FormatTomlSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &TomlSyntaxToken, f: &mut TomlFormatter) -> FormatResult<()> {
    FormatTomlSyntaxToken.format_removed(token, f)
}
