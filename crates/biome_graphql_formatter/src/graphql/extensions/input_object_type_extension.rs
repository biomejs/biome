use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputObjectTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputObjectTypeExtension;
impl FormatNodeRule<GraphqlInputObjectTypeExtension> for FormatGraphqlInputObjectTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlInputObjectTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
