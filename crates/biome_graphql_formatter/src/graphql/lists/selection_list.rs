use crate::prelude::*;
use biome_graphql_syntax::GraphqlSelectionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSelectionList;
impl FormatRule<GraphqlSelectionList> for FormatGraphqlSelectionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlSelectionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
