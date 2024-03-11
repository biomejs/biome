//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsArrowFunctionParameters;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsArrowFunctionParameters;
impl FormatRule<AnyJsArrowFunctionParameters> for FormatAnyJsArrowFunctionParameters {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsArrowFunctionParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsArrowFunctionParameters::AnyJsBinding(node) => node.format().fmt(f),
            AnyJsArrowFunctionParameters::JsParameters(node) => node.format().fmt(f),
        }
    }
}
