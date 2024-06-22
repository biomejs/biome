use crate::prelude::*;
use biome_graphql_syntax::GraphqlUnionTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionTypeExtension;
impl FormatNodeRule<GraphqlUnionTypeExtension> for FormatGraphqlUnionTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
