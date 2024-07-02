use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlSchemaExtension, GraphqlSchemaExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSchemaExtension;
impl FormatNodeRule<GraphqlSchemaExtension> for FormatGraphqlSchemaExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlSchemaExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlSchemaExtensionFields {
            extend_token,
            schema_token,
            directives,
            root_operation_types,
        } = node.as_fields();

        write!(
            f,
            [
                extend_token.format(),
                space(),
                schema_token.format(),
                directives.format(),
            ]
        )?;

        if let Some(root_operation_types) = root_operation_types {
            write!(f, [space(), root_operation_types.format(),])?;
        }

        Ok(())
    }
}
