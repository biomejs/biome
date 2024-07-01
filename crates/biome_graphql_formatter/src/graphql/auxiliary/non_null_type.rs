use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlNonNullType, GraphqlNonNullTypeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNonNullType;
impl FormatNodeRule<GraphqlNonNullType> for FormatGraphqlNonNullType {
    fn fmt_fields(&self, node: &GraphqlNonNullType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlNonNullTypeFields { base, excl_token } = node.as_fields();

        write!(f, [base.format(), excl_token.format()])
    }
}
