//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlDefinition;
impl FormatRule<AnyGraphqlDefinition> for FormatAnyGraphqlDefinition {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlDefinition, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlDefinition::AnyGraphqlOperationDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlBogusDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlEnumTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlEnumTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlFragmentDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlInputObjectTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlInputObjectTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlInterfaceTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlInterfaceTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlObjectTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlObjectTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlScalarTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlScalarTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlSchemaDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlSchemaExtension(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlUnionTypeDefinition(node) => node.format().fmt(f),
            AnyGraphqlDefinition::GraphqlUnionTypeExtension(node) => node.format().fmt(f),
        }
    }
}
