use crate::prelude::*;
use biome_graphql_syntax::GraphqlUnionMemberTypeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionMemberTypeList;
impl FormatRule<GraphqlUnionMemberTypeList> for FormatGraphqlUnionMemberTypeList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlUnionMemberTypeList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
