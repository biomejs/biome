use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlStringValue, GraphqlStringValueFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlStringValue;
impl FormatNodeRule<GraphqlStringValue> for FormatGraphqlStringValue {
    fn fmt_fields(&self, node: &GraphqlStringValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlStringValueFields {
            graphql_string_literal_token,
        } = node.as_fields();

        write![f, [graphql_string_literal_token.format()]]
    }
}
