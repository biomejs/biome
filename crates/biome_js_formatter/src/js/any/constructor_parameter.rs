//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsConstructorParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsConstructorParameter;
impl FormatRule<AnyJsConstructorParameter> for FormatAnyJsConstructorParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsConstructorParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsConstructorParameter::AnyJsFormalParameter(node) => node.format().fmt(f),
            AnyJsConstructorParameter::JsRestParameter(node) => node.format().fmt(f),
            AnyJsConstructorParameter::TsPropertyParameter(node) => node.format().fmt(f),
        }
    }
}
