use crate::prelude::*;
use biome_graphql_syntax::GraphqlNonNullType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNonNullType;
impl FormatNodeRule<GraphqlNonNullType> for FormatGraphqlNonNullType {
    fn fmt_fields(&self, node: &GraphqlNonNullType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
