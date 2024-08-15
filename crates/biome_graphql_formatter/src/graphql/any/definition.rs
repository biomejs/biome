//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlDefinition;
impl FormatRule<AnyGraphqlDefinition> for FormatAnyGraphqlDefinition {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlDefinition, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlBogusDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlFragmentDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlOperationDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlSchemaDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlSchemaExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlSelectionSet(node) => node.format().fmt(f),
        }
    }
}
