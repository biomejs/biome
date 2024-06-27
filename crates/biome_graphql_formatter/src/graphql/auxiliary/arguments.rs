use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlArguments, GraphqlArgumentsFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArguments;
impl FormatNodeRule<GraphqlArguments> for FormatGraphqlArguments {
    fn fmt_fields(&self, node: &GraphqlArguments, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlArgumentsFields {
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
