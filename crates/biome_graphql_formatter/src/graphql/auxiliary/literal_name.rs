use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlLiteralName, GraphqlLiteralNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlLiteralName;
impl FormatNodeRule<GraphqlLiteralName> for FormatGraphqlLiteralName {
    fn fmt_fields(&self, node: &GraphqlLiteralName, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlLiteralNameFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
