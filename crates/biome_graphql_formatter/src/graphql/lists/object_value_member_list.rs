use crate::prelude::*;
use biome_graphql_syntax::GraphqlObjectValueMemberList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectValueMemberList;
impl FormatRule<GraphqlObjectValueMemberList> for FormatGraphqlObjectValueMemberList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlObjectValueMemberList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
