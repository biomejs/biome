use crate::prelude::*;
use biome_graphql_syntax::GraphqlNullValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNullValue;
impl FormatNodeRule<GraphqlNullValue> for FormatGraphqlNullValue {
    fn fmt_fields(&self, node: &GraphqlNullValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
