use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlVariable, GraphqlVariableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariable;
impl FormatNodeRule<GraphqlVariable> for FormatGraphqlVariable {
    fn fmt_fields(&self, node: &GraphqlVariable, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlVariableFields { dollar_token, name } = node.as_fields();

        write![f, [dollar_token.format(), name.format(),]]
    }
}
