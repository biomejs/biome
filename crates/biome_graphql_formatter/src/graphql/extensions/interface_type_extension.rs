use crate::prelude::*;
use biome_graphql_syntax::GraphqlInterfaceTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInterfaceTypeExtension;
impl FormatNodeRule<GraphqlInterfaceTypeExtension> for FormatGraphqlInterfaceTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlInterfaceTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
