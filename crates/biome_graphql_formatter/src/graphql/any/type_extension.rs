//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlTypeExtension;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlTypeExtension;
impl FormatRule<AnyGraphqlTypeExtension> for FormatAnyGraphqlTypeExtension {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlTypeExtension, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(node) => node.format().fmt(f),
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(node) => node.format().fmt(f),
        }
    }
}
