use crate::prelude::*;
use biome_graphql_syntax::GraphqlRootOperationTypeDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypeDefinitionList;
impl FormatRule<GraphqlRootOperationTypeDefinitionList>
    for FormatGraphqlRootOperationTypeDefinitionList
{
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlRootOperationTypeDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for operation_type in node {
            join.entry(
                operation_type.syntax(),
                &format_or_verbatim(operation_type.format()),
            );
        }

        join.finish()
    }
}
