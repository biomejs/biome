//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsName;
impl FormatRule<AnyJsName> for FormatAnyJsName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsName::JsMetavariable(node) => node.format().fmt(f),
            AnyJsName::JsName(node) => node.format().fmt(f),
            AnyJsName::JsPrivateName(node) => node.format().fmt(f),
        }
    }
}
