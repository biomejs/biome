use crate::prelude::*;
use biome_graphql_syntax::GraphqlFieldDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldDefinitionList;
impl FormatRule<GraphqlFieldDefinitionList> for FormatGraphqlFieldDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlFieldDefinitionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for definition in node {
            join.entry(
                definition.syntax(),
                &format_or_verbatim(definition.format()),
            );
        }

        join.finish()
    }
}
