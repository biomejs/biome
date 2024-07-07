use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlEnumValuesDefinition, GraphqlEnumValuesDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValuesDefinition;
impl FormatNodeRule<GraphqlEnumValuesDefinition> for FormatGraphqlEnumValuesDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumValuesDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlEnumValuesDefinitionFields {
            l_curly_token,
            values,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                block_indent(&values.format()),
                r_curly_token.format(),
            ]
        )
    }
}
