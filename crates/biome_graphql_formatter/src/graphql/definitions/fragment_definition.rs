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
            description,
            fragment_token,
            name,
            variables,
            type_condition,
            directives,
            selection_set,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break()])?;
        }

        write!(
            f,
            [
                fragment_token.format(),
                space(),
                name.format(),
                variables.format(),
                space(),
                type_condition.format(),
                directives.format(),
                space(),
                selection_set.format(),
            ]
        )
    }
}
