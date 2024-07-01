use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlFragmentSpread, GraphqlFragmentSpreadFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFragmentSpread;
impl FormatNodeRule<GraphqlFragmentSpread> for FormatGraphqlFragmentSpread {
    fn fmt_fields(
        &self,
        node: &GraphqlFragmentSpread,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlFragmentSpreadFields {
            dotdotdot_token,
            name,
            directives,
        } = node.as_fields();

        write![
            f,
            [dotdotdot_token.format(), name.format(), directives.format()]
        ]
    }
}
