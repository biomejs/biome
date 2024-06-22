use crate::prelude::*;
use biome_graphql_syntax::GraphqlDirectiveLocationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveLocationList;
impl FormatRule<GraphqlDirectiveLocationList> for FormatGraphqlDirectiveLocationList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlDirectiveLocationList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
