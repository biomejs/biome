use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlFloatValue, GraphqlFloatValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFloatValue;
impl FormatNodeRule<GraphqlFloatValue> for FormatGraphqlFloatValue {
    fn fmt_fields(&self, node: &GraphqlFloatValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlFloatValueFields {
            graphql_float_literal_token,
        } = node.as_fields();

        write![f, [graphql_float_literal_token.format()]]
    }
}
