use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlObjectTypeDefinition, GraphqlObjectTypeDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectTypeDefinition;
impl FormatNodeRule<GraphqlObjectTypeDefinition> for FormatGraphqlObjectTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlObjectTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlObjectTypeDefinitionFields {
            description,
            type_token,
            name,
            implements,
            directives,
            fields,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                type_token.format(),
                space(),
                name.format(),
                space(),
                implements.format(),
                directives.format(),
                space(),
                fields.format(),
            ]
        )
    }
}
