use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlIntValue, GraphqlIntValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlIntValue;
impl FormatNodeRule<GraphqlIntValue> for FormatGraphqlIntValue {
    fn fmt_fields(&self, node: &GraphqlIntValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlIntValueFields {
            graphql_int_literal_token,
        } = node.as_fields();

        write![f, [graphql_int_literal_token.format()]]
    }
}
