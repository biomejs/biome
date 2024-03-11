//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxChild;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxChild;
impl FormatRule<AnyJsxChild> for FormatAnyJsxChild {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxChild, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxChild::JsxElement(node) => node.format().fmt(f),
            AnyJsxChild::JsxExpressionChild(node) => node.format().fmt(f),
            AnyJsxChild::JsxFragment(node) => node.format().fmt(f),
            AnyJsxChild::JsxSelfClosingElement(node) => node.format().fmt(f),
            AnyJsxChild::JsxSpreadChild(node) => node.format().fmt(f),
            AnyJsxChild::JsxText(node) => node.format().fmt(f),
        }
    }
}
