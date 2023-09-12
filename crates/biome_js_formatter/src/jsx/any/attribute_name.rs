//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxAttributeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxAttributeName;
impl FormatRule<AnyJsxAttributeName> for FormatAnyJsxAttributeName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxAttributeName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxAttributeName::JsxName(node) => node.format().fmt(f),
            AnyJsxAttributeName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
