//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxAttributeValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxAttributeValue;
impl FormatRule<AnyJsxAttributeValue> for FormatAnyJsxAttributeValue {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxAttributeValue, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxAttributeValue::AnyJsxTag(node) => node.format().fmt(f),
            AnyJsxAttributeValue::JsxExpressionAttributeValue(node) => node.format().fmt(f),
            AnyJsxAttributeValue::JsxString(node) => node.format().fmt(f),
        }
    }
}
