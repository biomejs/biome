//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsParameter;
impl FormatRule<AnyJsParameter> for FormatAnyJsParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsParameter::AnyJsFormalParameter(node) => node.format().fmt(f),
            AnyJsParameter::JsRestParameter(node) => node.format().fmt(f),
            AnyJsParameter::TsThisParameter(node) => node.format().fmt(f),
        }
    }
}
