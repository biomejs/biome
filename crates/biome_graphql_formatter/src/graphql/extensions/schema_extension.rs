use crate::prelude::*;
use biome_graphql_syntax::GraphqlSchemaExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSchemaExtension;
impl FormatNodeRule<GraphqlSchemaExtension> for FormatGraphqlSchemaExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlSchemaExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
