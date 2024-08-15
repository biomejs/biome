//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlValue;
impl FormatRule<AnyGraphqlValue> for FormatAnyGraphqlValue {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlValue::GraphqlBogusValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlBooleanValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlEnumValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlFloatValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlIntValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlListValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlNullValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlObjectValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlStringValue(node) => node.format().fmt(f),
            AnyGraphqlValue::GraphqlVariableReference(node) => node.format().fmt(f),
        }
    }
}
