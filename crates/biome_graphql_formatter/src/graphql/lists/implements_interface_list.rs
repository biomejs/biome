use crate::prelude::*;
use crate::utils::list::write_interface_like_list;
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
        write_interface_like_list(node, f)
    }
}
