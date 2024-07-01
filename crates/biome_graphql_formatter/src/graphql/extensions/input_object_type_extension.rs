use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{
    GraphqlInputObjectTypeExtension, GraphqlInputObjectTypeExtensionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputObjectTypeExtension;
impl FormatNodeRule<GraphqlInputObjectTypeExtension> for FormatGraphqlInputObjectTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlInputObjectTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInputObjectTypeExtensionFields {
            extend_token,
            input_token,
            name,
            directives,
            input_fields,
        } = node.as_fields();

        write![
            f,
            [
                extend_token.format(),
                space(),
                input_token.format(),
                space(),
                name.format(),
                directives.format(),
                space(),
                input_fields.format(),
            ]
        ]
    }
}
