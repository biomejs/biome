use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlScalarTypeExtension, GraphqlScalarTypeExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlScalarTypeExtension;
impl FormatNodeRule<GraphqlScalarTypeExtension> for FormatGraphqlScalarTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlScalarTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlScalarTypeExtensionFields {
            extend_token,
            scalar_token,
            name,
            directives,
        } = node.as_fields();

        write![
            f,
            [
                extend_token.format(),
                space(),
                scalar_token.format(),
                space(),
                name.format(),
                directives.format(),
            ]
        ]
    }
}
