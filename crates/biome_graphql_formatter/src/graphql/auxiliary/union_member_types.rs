use crate::prelude::*;
use biome_graphql_syntax::GraphqlUnionMemberTypes;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionMemberTypes;
impl FormatNodeRule<GraphqlUnionMemberTypes> for FormatGraphqlUnionMemberTypes {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionMemberTypes,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
