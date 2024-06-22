use crate::prelude::*;
use biome_graphql_syntax::GraphqlImplementsInterfaceList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlImplementsInterfaceList;
impl FormatRule<GraphqlImplementsInterfaceList> for FormatGraphqlImplementsInterfaceList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlImplementsInterfaceList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
