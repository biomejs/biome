//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsObjectMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsObjectMemberName;
impl FormatRule<AnyJsObjectMemberName> for FormatAnyJsObjectMemberName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsObjectMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsObjectMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            AnyJsObjectMemberName::JsComputedMemberName(node) => node.format().fmt(f),
        }
    }
}
