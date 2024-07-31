//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExpression;
impl FormatRule<AnyJsExpression> for FormatAnyJsExpression {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsExpression::AnyJsLiteralExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsArrayExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsArrowFunctionExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsAssignmentExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsAwaitExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsBinaryExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsBogusExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsCallExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsClassExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsComputedMemberExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsConditionalExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsFunctionExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsIdentifierExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsImportCallExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsImportMetaExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsInExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsInstanceofExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsLogicalExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsMetavariable(node) => node.format().fmt(f),
            AnyJsExpression::JsNewExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsNewTargetExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsObjectExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsParenthesizedExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsPostUpdateExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsPreUpdateExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsSequenceExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsStaticMemberExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsSuperExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsTemplateExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsThisExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsUnaryExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsYieldExpression(node) => node.format().fmt(f),
            AnyJsExpression::JsxTagExpression(node) => node.format().fmt(f),
            AnyJsExpression::TsAsExpression(node) => node.format().fmt(f),
            AnyJsExpression::TsInstantiationExpression(node) => node.format().fmt(f),
            AnyJsExpression::TsNonNullAssertionExpression(node) => node.format().fmt(f),
            AnyJsExpression::TsSatisfiesExpression(node) => node.format().fmt(f),
            AnyJsExpression::TsTypeAssertionExpression(node) => node.format().fmt(f),
        }
    }
}
