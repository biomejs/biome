use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlDefaultValue, GraphqlDefaultValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDefaultValue;
impl FormatNodeRule<GraphqlDefaultValue> for FormatGraphqlDefaultValue {
    fn fmt_fields(&self, node: &GraphqlDefaultValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlDefaultValueFields { eq_token, value } = node.as_fields();

        write!(f, [space(), eq_token.format(), space(), value.format(),])
    }
}
