use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlArgumentsDefinition, GraphqlArgumentsDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgumentsDefinition;
impl FormatNodeRule<GraphqlArgumentsDefinition> for FormatGraphqlArgumentsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlArgumentsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlArgumentsDefinitionFields {
            l_paren_token,
            arguments,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                l_paren_token.format(),
                soft_block_indent(&arguments.format()),
                r_paren_token.format(),
            ))]
        )
    }
}
