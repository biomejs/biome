use crate::prelude::*;
use biome_graphql_syntax::GraphqlDirectiveLocation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveLocation;
impl FormatNodeRule<GraphqlDirectiveLocation> for FormatGraphqlDirectiveLocation {
    fn fmt_fields(
        &self,
        node: &GraphqlDirectiveLocation,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
