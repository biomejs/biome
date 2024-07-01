use crate::prelude::*;
use biome_formatter::format_args;
use biome_graphql_syntax::GraphqlVariableDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinitionList;
impl FormatRule<GraphqlVariableDefinitionList> for FormatGraphqlVariableDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlVariableDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join_with(&format_args!(
            if_group_fits_on_line(&format_args![text(","), space()]),
            soft_line_break(),
        ))
        .entries(node.iter().formatted())
        .finish()
    }
}
