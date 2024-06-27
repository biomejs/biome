use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlOperationDefinition, GraphqlOperationDefinitionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlOperationDefinition;
impl FormatNodeRule<GraphqlOperationDefinition> for FormatGraphqlOperationDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlOperationDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlOperationDefinitionFields {
            ty,
            name,
            variables,
            directives,
            selection_set,
        } = node.as_fields();

        write!(f, [ty.format()])?;

        if let Some(name) = name.as_ref() {
            // add space between operation type and name
            write!(f, [space(), name.format()])?;
        } else if variables.is_some() {
            // if we don't have a name but we have variables, add a space
            // between the operation type and the variables
            write!(f, [space()])?;
        }

        write!(
            f,
            [
                variables.format(),
                directives.format(),
                space(),
                selection_set.format(),
            ]
        )
    }
}
