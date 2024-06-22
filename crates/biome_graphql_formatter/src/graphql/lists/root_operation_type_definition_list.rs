use crate::prelude::*;
use biome_graphql_syntax::GraphqlRootOperationTypeDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypeDefinitionList;
impl FormatRule<GraphqlRootOperationTypeDefinitionList>
    for FormatGraphqlRootOperationTypeDefinitionList
{
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlRootOperationTypeDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
