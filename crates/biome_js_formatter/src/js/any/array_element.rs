//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsArrayElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsArrayElement;
impl FormatRule<AnyJsArrayElement> for FormatAnyJsArrayElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsArrayElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsArrayElement::AnyJsExpression(node) => node.format().fmt(f),
            AnyJsArrayElement::JsArrayHole(node) => node.format().fmt(f),
            AnyJsArrayElement::JsSpread(node) => node.format().fmt(f),
        }
    }
}
