//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxTag;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxTag;
impl FormatRule<AnyJsxTag> for FormatAnyJsxTag {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxTag, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxTag::JsxElement(node) => node.format().fmt(f),
            AnyJsxTag::JsxFragment(node) => node.format().fmt(f),
            AnyJsxTag::JsxSelfClosingElement(node) => node.format().fmt(f),
        }
    }
}
