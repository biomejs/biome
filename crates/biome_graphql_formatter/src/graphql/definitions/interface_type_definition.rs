use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlInterfaceTypeDefinition, GraphqlInterfaceTypeDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInterfaceTypeDefinition;
impl FormatNodeRule<GraphqlInterfaceTypeDefinition> for FormatGraphqlInterfaceTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInterfaceTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInterfaceTypeDefinitionFields {
            description,
            interface_token,
            name,
            implements,
            directives,
            fields,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(f, [interface_token.format(), space(), name.format(),])?;

        if let Some(implements) = implements {
            write!(f, [space(), implements.format(),])?;
        }

        write!(f, [directives.format()])?;

        if let Some(fields) = fields {
            write!(f, [space(), fields.format(),])?;
        }

        Ok(())
    }
}
