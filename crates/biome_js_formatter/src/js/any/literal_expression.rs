//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsLiteralExpression;
impl FormatRule<AnyJsLiteralExpression> for FormatAnyJsLiteralExpression {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsLiteralExpression::JsBigintLiteralExpression(node) => node.format().fmt(f),
            AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => node.format().fmt(f),
            AnyJsLiteralExpression::JsNullLiteralExpression(node) => node.format().fmt(f),
            AnyJsLiteralExpression::JsNumberLiteralExpression(node) => node.format().fmt(f),
            AnyJsLiteralExpression::JsRegexLiteralExpression(node) => node.format().fmt(f),
            AnyJsLiteralExpression::JsStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
