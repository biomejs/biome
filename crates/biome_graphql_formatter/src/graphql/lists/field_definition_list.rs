use crate::prelude::*;
use biome_graphql_syntax::GraphqlFieldDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldDefinitionList;
impl FormatRule<GraphqlFieldDefinitionList> for FormatGraphqlFieldDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlFieldDefinitionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
