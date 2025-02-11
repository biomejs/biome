use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlObjectValue, GraphqlObjectValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectValue;
impl FormatNodeRule<GraphqlObjectValue> for FormatGraphqlObjectValue {
    fn fmt_fields(&self, node: &GraphqlObjectValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlObjectValueFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                soft_block_indent_with_maybe_space(
                    &members.format(),
                    f.options().bracket_spacing().value()
                ),
                r_curly_token.format()
            ])]
        )
    }
}
