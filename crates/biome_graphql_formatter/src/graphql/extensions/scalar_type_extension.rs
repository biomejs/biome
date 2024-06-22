use crate::prelude::*;
use biome_graphql_syntax::GraphqlScalarTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlScalarTypeExtension;
impl FormatNodeRule<GraphqlScalarTypeExtension> for FormatGraphqlScalarTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlScalarTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
