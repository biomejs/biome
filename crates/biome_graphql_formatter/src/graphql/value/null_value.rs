use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlNullValue, GraphqlNullValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNullValue;
impl FormatNodeRule<GraphqlNullValue> for FormatGraphqlNullValue {
    fn fmt_fields(&self, node: &GraphqlNullValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlNullValueFields { null_token } = node.as_fields();

        write![f, [null_token.format()]]
    }
}
