use crate::prelude::*;
use biome_graphql_syntax::GraphqlListValueElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListValueElementList;
impl FormatRule<GraphqlListValueElementList> for FormatGraphqlListValueElementList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlListValueElementList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
