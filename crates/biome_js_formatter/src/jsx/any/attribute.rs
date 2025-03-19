//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxAttribute;
impl FormatRule<AnyJsxAttribute> for FormatAnyJsxAttribute {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxAttribute::JsMetavariable(node) => node.format().fmt(f),
            AnyJsxAttribute::JsxAttribute(node) => node.format().fmt(f),
            AnyJsxAttribute::JsxSpreadAttribute(node) => node.format().fmt(f),
        }
    }
}
