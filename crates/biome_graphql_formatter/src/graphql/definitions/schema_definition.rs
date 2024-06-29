use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlSchemaDefinition, GraphqlSchemaDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSchemaDefinition;
impl FormatNodeRule<GraphqlSchemaDefinition> for FormatGraphqlSchemaDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlSchemaDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlSchemaDefinitionFields {
            description,
            schema_token,
            directives,
            root_operation_types,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                schema_token.format(),
                directives.format(),
                space(),
                root_operation_types.format(),
            ]
        )
    }
}
