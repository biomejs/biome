use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlEnumTypeExtension, GraphqlEnumTypeExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumTypeExtension;
impl FormatNodeRule<GraphqlEnumTypeExtension> for FormatGraphqlEnumTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlEnumTypeExtensionFields {
            extend_token,
            enum_token,
            name,
            directives,
            enum_values,
        } = node.as_fields();

        write![
            f,
            [
                extend_token.format(),
                space(),
                enum_token.format(),
                space(),
                name.format(),
                directives.format(),
                space(),
                enum_values.format(),
            ]
        ]
    }
}
