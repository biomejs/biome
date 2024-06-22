use crate::prelude::*;
use biome_graphql_syntax::GraphqlObjectTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectTypeExtension;
impl FormatNodeRule<GraphqlObjectTypeExtension> for FormatGraphqlObjectTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlObjectTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
