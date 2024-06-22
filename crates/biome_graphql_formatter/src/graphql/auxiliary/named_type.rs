use crate::prelude::*;
use biome_graphql_syntax::GraphqlNamedType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNamedType;
impl FormatNodeRule<GraphqlNamedType> for FormatGraphqlNamedType {
    fn fmt_fields(&self, node: &GraphqlNamedType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
