use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlInterfaceTypeExtension, GraphqlInterfaceTypeExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInterfaceTypeExtension;
impl FormatNodeRule<GraphqlInterfaceTypeExtension> for FormatGraphqlInterfaceTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlInterfaceTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInterfaceTypeExtensionFields {
            extend_token,
            interface_token,
            name,
            implements,
            directives,
            fields,
        } = node.as_fields();

        write!(
            f,
            [
                extend_token.format(),
                space(),
                interface_token.format(),
                space(),
                name.format(),
            ]
        )?;

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
