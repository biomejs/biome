use crate::prelude::*;
use biome_graphql_syntax::GraphqlArgumentDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgumentDefinitionList;
impl FormatRule<GraphqlArgumentDefinitionList> for FormatGraphqlArgumentDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlArgumentDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
