use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlObjectTypeExtension, GraphqlObjectTypeExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectTypeExtension;
impl FormatNodeRule<GraphqlObjectTypeExtension> for FormatGraphqlObjectTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlObjectTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlObjectTypeExtensionFields {
            extend_token,
            type_token,
            name,
            implements,
            directives,
            fields,
        } = node.as_fields();

        write![
            f,
            [
                extend_token.format(),
                space(),
                type_token.format(),
                space(),
                name.format(),
            ]
        ]?;

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
