use crate::context::GritFormatContext;
use crate::cst::FormatGritSyntaxToken;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Format, FormatResult};
use biome_grit_syntax::GritSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a GritSyntaxToken,
}

pub(crate) fn format_removed(token: &GritSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<GritFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<GritFormatContext>) -> FormatResult<()> {
        FormatGritSyntaxToken.format_removed(self.token, f)
    }
}
