use crate::FormatGraphqlSyntaxToken;
use crate::prelude::GraphqlFormatContext;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Format, FormatResult};
use biome_graphql_syntax::GraphqlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a GraphqlSyntaxToken,
}

pub(crate) fn format_removed(token: &GraphqlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<GraphqlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<GraphqlFormatContext>) -> FormatResult<()> {
        FormatGraphqlSyntaxToken.format_removed(self.token, f)
    }
}
