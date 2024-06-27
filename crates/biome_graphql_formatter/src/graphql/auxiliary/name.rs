use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlName, GraphqlNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlName;
impl FormatNodeRule<GraphqlName> for FormatGraphqlName {
    fn fmt_fields(&self, node: &GraphqlName, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
