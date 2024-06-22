use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputFieldList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputFieldList;
impl FormatRule<GraphqlInputFieldList> for FormatGraphqlInputFieldList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlInputFieldList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
