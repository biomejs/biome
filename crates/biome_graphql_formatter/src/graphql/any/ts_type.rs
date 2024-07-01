//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlType;
impl FormatRule<AnyGraphqlType> for FormatAnyGraphqlType {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlType::AnyGraphqlPrimitiveType(node) => node.format().fmt(f),
            AnyGraphqlType::GraphqlBogusType(node) => node.format().fmt(f),
            AnyGraphqlType::GraphqlNonNullType(node) => node.format().fmt(f),
        }
    }
}
