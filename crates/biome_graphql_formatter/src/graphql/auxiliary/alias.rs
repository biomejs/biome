use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlAlias, GraphqlAliasFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlAlias;
impl FormatNodeRule<GraphqlAlias> for FormatGraphqlAlias {
    fn fmt_fields(&self, node: &GraphqlAlias, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlAliasFields { value, colon_token } = node.as_fields();

        write!(f, [value.format(), colon_token.format(), space()])
    }
}
