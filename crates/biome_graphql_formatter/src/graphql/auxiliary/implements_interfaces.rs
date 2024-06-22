use crate::prelude::*;
use biome_graphql_syntax::GraphqlImplementsInterfaces;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlImplementsInterfaces;
impl FormatNodeRule<GraphqlImplementsInterfaces> for FormatGraphqlImplementsInterfaces {
    fn fmt_fields(
        &self,
        node: &GraphqlImplementsInterfaces,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
