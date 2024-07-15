//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlPrimitiveType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlPrimitiveType;
impl FormatRule<AnyGraphqlPrimitiveType> for FormatAnyGraphqlPrimitiveType {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlPrimitiveType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlPrimitiveType::GraphqlListType(node) => node.format().fmt(f),
            AnyGraphqlPrimitiveType::GraphqlNameReference(node) => node.format().fmt(f),
        }
    }
}
