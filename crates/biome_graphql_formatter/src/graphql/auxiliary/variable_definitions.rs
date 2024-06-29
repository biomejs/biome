use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlVariableDefinitions, GraphqlVariableDefinitionsFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinitions;
impl FormatNodeRule<GraphqlVariableDefinitions> for FormatGraphqlVariableDefinitions {
    fn fmt_fields(
        &self,
        node: &GraphqlVariableDefinitions,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlVariableDefinitionsFields {
            l_paren_token,
            elements,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                l_paren_token.format(),
                soft_block_indent(&elements.format()),
                r_paren_token.format(),
            ))]
        )
    }
}
