use crate::prelude::*;
use biome_graphql_syntax::GraphqlDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDefinitionList;
impl FormatRule<GraphqlDefinitionList> for FormatGraphqlDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlDefinitionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
