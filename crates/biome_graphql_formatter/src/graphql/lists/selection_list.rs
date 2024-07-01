use crate::prelude::*;
use biome_graphql_syntax::GraphqlSelectionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSelectionList;
impl FormatRule<GraphqlSelectionList> for FormatGraphqlSelectionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlSelectionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
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
