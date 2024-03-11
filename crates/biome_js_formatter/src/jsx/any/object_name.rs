//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsxObjectName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxObjectName;
impl FormatRule<AnyJsxObjectName> for FormatAnyJsxObjectName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxObjectName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxObjectName::JsxMemberName(node) => node.format().fmt(f),
            AnyJsxObjectName::JsxNamespaceName(node) => node.format().fmt(f),
            AnyJsxObjectName::JsxReferenceIdentifier(node) => node.format().fmt(f),
        }
    }
}
