use crate::prelude::*;
use biome_graphql_syntax::GraphqlVariableDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinitionList;
impl FormatRule<GraphqlVariableDefinitionList> for FormatGraphqlVariableDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlVariableDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
