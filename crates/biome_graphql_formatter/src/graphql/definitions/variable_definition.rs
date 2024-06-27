use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlVariableDefinition, GraphqlVariableDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinition;
impl FormatNodeRule<GraphqlVariableDefinition> for FormatGraphqlVariableDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlVariableDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlVariableDefinitionFields {
            variable,
            colon_token,
            ty,
            default,
            directives,
        } = node.as_fields();

        write!(
            f,
            [
                variable.format(),
                colon_token.format(),
                space(),
                ty.format(),
                default.format(),
                directives.format(),
            ]
        )
    }
}
