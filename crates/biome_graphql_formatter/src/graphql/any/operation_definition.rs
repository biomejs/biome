//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlOperationDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlOperationDefinition;
impl FormatRule<AnyGraphqlOperationDefinition> for FormatAnyGraphqlOperationDefinition {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &AnyGraphqlOperationDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(node) => node.format().fmt(f),
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(node) => node.format().fmt(f),
        }
    }
}
