//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsFunction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsFunction;
impl FormatRule<AnyJsFunction> for FormatAnyJsFunction {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsFunction, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsFunction::JsArrowFunctionExpression(node) => node.format().fmt(f),
            AnyJsFunction::JsFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(node) => node.format().fmt(f),
            AnyJsFunction::JsFunctionExpression(node) => node.format().fmt(f),
        }
    }
}
