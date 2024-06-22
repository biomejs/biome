use crate::prelude::*;
use biome_graphql_syntax::GraphqlArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgumentList;
impl FormatRule<GraphqlArgumentList> for FormatGraphqlArgumentList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlArgumentList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
