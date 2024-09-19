//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsFormalParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsFormalParameter;
impl FormatRule<AnyJsFormalParameter> for FormatAnyJsFormalParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsFormalParameter::JsBogusParameter(node) => node.format().fmt(f),
            AnyJsFormalParameter::JsFormalParameter(node) => node.format().fmt(f),
            AnyJsFormalParameter::JsMetavariable(node) => node.format().fmt(f),
        }
    }
}
