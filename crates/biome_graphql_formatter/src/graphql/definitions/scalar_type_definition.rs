use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlScalarTypeDefinition, GraphqlScalarTypeDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlScalarTypeDefinition;
impl FormatNodeRule<GraphqlScalarTypeDefinition> for FormatGraphqlScalarTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlScalarTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlScalarTypeDefinitionFields {
            description,
            scalar_token,
            name,
            directives,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                scalar_token.format(),
                space(),
                name.format(),
                directives.format(),
            ]
        )
    }
}
