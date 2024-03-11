//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_json_syntax::AnyJsonValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsonValue;
impl FormatRule<AnyJsonValue> for FormatAnyJsonValue {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &AnyJsonValue, f: &mut JsonFormatter) -> FormatResult<()> {
        match node {
            AnyJsonValue::JsonArrayValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonBogusValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonBooleanValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonNullValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonNumberValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonObjectValue(node) => node.format().fmt(f),
            AnyJsonValue::JsonStringValue(node) => node.format().fmt(f),
        }
    }
}
