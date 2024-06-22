use crate::prelude::*;
use biome_graphql_syntax::GraphqlListType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListType;
impl FormatNodeRule<GraphqlListType> for FormatGraphqlListType {
    fn fmt_fields(&self, node: &GraphqlListType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
