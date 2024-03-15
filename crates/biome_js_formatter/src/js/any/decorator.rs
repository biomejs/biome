//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsDecorator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsDecorator;
impl FormatRule<AnyJsDecorator> for FormatAnyJsDecorator {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsDecorator, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsDecorator::JsBogusExpression(node) => node.format().fmt(f),
            AnyJsDecorator::JsCallExpression(node) => node.format().fmt(f),
            AnyJsDecorator::JsIdentifierExpression(node) => node.format().fmt(f),
            AnyJsDecorator::JsParenthesizedExpression(node) => node.format().fmt(f),
            AnyJsDecorator::JsStaticMemberExpression(node) => node.format().fmt(f),
        }
    }
}
