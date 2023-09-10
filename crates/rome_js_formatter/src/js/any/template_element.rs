//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsTemplateElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsTemplateElement;
impl FormatRule<AnyJsTemplateElement> for FormatAnyJsTemplateElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsTemplateElement::JsTemplateChunkElement(node) => node.format().fmt(f),
            AnyJsTemplateElement::JsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
