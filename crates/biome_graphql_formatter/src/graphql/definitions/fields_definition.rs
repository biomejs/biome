use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlFieldsDefinition, GraphqlFieldsDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldsDefinition;
impl FormatNodeRule<GraphqlFieldsDefinition> for FormatGraphqlFieldsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlFieldsDefinitionFields {
            l_curly_token,
            fields,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                block_indent(&fields.format()),
                r_curly_token.format(),
            ]
        )
    }
}
