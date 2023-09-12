//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsTemplateElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsTemplateElement;
impl FormatRule<AnyTsTemplateElement> for FormatAnyTsTemplateElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsTemplateElement::TsTemplateChunkElement(node) => node.format().fmt(f),
            AnyTsTemplateElement::TsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
