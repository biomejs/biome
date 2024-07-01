use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlEnumTypeDefinition, GraphqlEnumTypeDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumTypeDefinition;
impl FormatNodeRule<GraphqlEnumTypeDefinition> for FormatGraphqlEnumTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlEnumTypeDefinitionFields {
            description,
            enum_token,
            name,
            directives,
            enum_values,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                enum_token.format(),
                space(),
                name.format(),
                directives.format(),
                space(),
                enum_values.format(),
            ]
        )
    }
}
