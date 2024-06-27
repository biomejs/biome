use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlListValue, GraphqlListValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListValue;
impl FormatNodeRule<GraphqlListValue> for FormatGraphqlListValue {
    fn fmt_fields(&self, node: &GraphqlListValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlListValueFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write![
            f,
            [group(&format_args!(
                l_brack_token.format(),
                soft_block_indent(&elements.format()),
                r_brack_token.format()
            ))]
        ]
    }
}
