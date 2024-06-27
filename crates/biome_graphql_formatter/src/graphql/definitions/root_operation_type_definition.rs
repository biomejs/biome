use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{
    GraphqlRootOperationTypeDefinition, GraphqlRootOperationTypeDefinitionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypeDefinition;
impl FormatNodeRule<GraphqlRootOperationTypeDefinition>
    for FormatGraphqlRootOperationTypeDefinition
{
    fn fmt_fields(
        &self,
        node: &GraphqlRootOperationTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlRootOperationTypeDefinitionFields {
            operation_type,
            colon_token,
            named_type,
        } = node.as_fields();

        write!(
            f,
            [
                operation_type.format(),
                colon_token.format(),
                space(),
                named_type.format(),
            ]
        )
    }
}
