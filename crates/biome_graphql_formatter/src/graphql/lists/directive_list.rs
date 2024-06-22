use crate::prelude::*;
use biome_graphql_syntax::GraphqlDirectiveList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveList;
impl FormatRule<GraphqlDirectiveList> for FormatGraphqlDirectiveList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlDirectiveList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
