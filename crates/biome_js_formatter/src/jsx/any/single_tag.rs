//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxSingleTag;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxSingleTag;
impl FormatRule<AnyJsxSingleTag> for FormatAnyJsxSingleTag {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxSingleTag, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxSingleTag::JsBogus(node) => node.format().fmt(f),
            AnyJsxSingleTag::JsxClosingElement(node) => node.format().fmt(f),
            AnyJsxSingleTag::JsxClosingFragment(node) => node.format().fmt(f),
            AnyJsxSingleTag::JsxOpeningElement(node) => node.format().fmt(f),
            AnyJsxSingleTag::JsxOpeningFragment(node) => node.format().fmt(f),
            AnyJsxSingleTag::JsxSelfClosingElement(node) => node.format().fmt(f),
        }
    }
}
