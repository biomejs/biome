use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlInputFieldsDefinition, GraphqlInputFieldsDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputFieldsDefinition;
impl FormatNodeRule<GraphqlInputFieldsDefinition> for FormatGraphqlInputFieldsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInputFieldsDefinitionFields {
            l_curly_token,
            fields,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                l_curly_token.format(),
                block_indent(&fields.format()),
                r_curly_token.format()
            ]
        ]
    }
}
