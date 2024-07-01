use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{
    GraphqlInputObjectTypeDefinition, GraphqlInputObjectTypeDefinitionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputObjectTypeDefinition;
impl FormatNodeRule<GraphqlInputObjectTypeDefinition> for FormatGraphqlInputObjectTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputObjectTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInputObjectTypeDefinitionFields {
            description,
            input_token,
            name,
            directives,
            input_fields,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                input_token.format(),
                space(),
                name.format(),
                directives.format(),
                space(),
                input_fields.format(),
            ]
        )
    }
}
