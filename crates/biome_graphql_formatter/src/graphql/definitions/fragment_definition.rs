use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlFragmentDefinition, GraphqlFragmentDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFragmentDefinition;
impl FormatNodeRule<GraphqlFragmentDefinition> for FormatGraphqlFragmentDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFragmentDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlFragmentDefinitionFields {
            fragment_token,
            name,
            type_condition,
            directives,
            selection_set,
        } = node.as_fields();

        write!(
            f,
            [
                fragment_token.format(),
                space(),
                name.format(),
                space(),
                type_condition.format(),
                directives.format(),
                space(),
                selection_set.format(),
            ]
        )
    }
}
