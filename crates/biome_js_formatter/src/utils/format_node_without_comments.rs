use crate::js::auxiliary::metavariable::FormatJsMetavariable;
use crate::js::bogus::bogus_expression::FormatJsBogusExpression;
use crate::js::expressions::array_expression::FormatJsArrayExpression;
use crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression;
use crate::js::expressions::assignment_expression::FormatJsAssignmentExpression;
use crate::js::expressions::await_expression::FormatJsAwaitExpression;
use crate::js::expressions::bigint_literal_expression::FormatJsBigintLiteralExpression;
use crate::js::expressions::binary_expression::FormatJsBinaryExpression;
use crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression;
use crate::js::expressions::call_expression::FormatJsCallExpression;
use crate::js::expressions::class_expression::FormatJsClassExpression;
use crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression;
use crate::js::expressions::conditional_expression::FormatJsConditionalExpression;
use crate::js::expressions::function_expression::FormatJsFunctionExpression;
use crate::js::expressions::import_call_expression::FormatJsImportCallExpression;
use crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression;
use crate::js::expressions::in_expression::FormatJsInExpression;
use crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression;
use crate::js::expressions::logical_expression::FormatJsLogicalExpression;
use crate::js::expressions::new_expression::FormatJsNewExpression;
use crate::js::expressions::new_target_expression::FormatJsNewTargetExpression;
use crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression;
use crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression;
use crate::js::expressions::object_expression::FormatJsObjectExpression;
use crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression;
use crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression;
use crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression;
use crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression;
use crate::js::expressions::sequence_expression::FormatJsSequenceExpression;
use crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression;
use crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression;
use crate::js::expressions::super_expression::FormatJsSuperExpression;
use crate::js::expressions::template_expression::FormatJsTemplateExpression;
use crate::js::expressions::this_expression::FormatJsThisExpression;
use crate::js::expressions::unary_expression::FormatJsUnaryExpression;
use crate::js::expressions::yield_expression::FormatJsYieldExpression;
use crate::jsx::expressions::tag_expression::FormatJsxTagExpression;
use crate::ts::expressions::as_expression::FormatTsAsExpression;
use crate::ts::expressions::instantiation_expression::FormatTsInstantiationExpression;
use crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression;
use crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression;
use crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression;
use crate::{js::expressions::identifier_expression::FormatJsIdentifierExpression, prelude::*};
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExpressionWithoutComments;

impl FormatRule<AnyJsExpression> for FormatAnyJsExpressionWithoutComments {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsExpression::AnyJsLiteralExpression(literal_expr) => match literal_expr {
                AnyJsLiteralExpression::JsBigintLiteralExpression(node) => {
                    FormatJsBigintLiteralExpression.fmt_node(node, f)
                }
                AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => {
                    FormatJsBooleanLiteralExpression.fmt_node(node, f)
                }
                AnyJsLiteralExpression::JsNullLiteralExpression(node) => {
                    FormatJsNullLiteralExpression.fmt_node(node, f)
                }
                AnyJsLiteralExpression::JsNumberLiteralExpression(node) => {
                    FormatJsNumberLiteralExpression.fmt_node(node, f)
                }
                AnyJsLiteralExpression::JsRegexLiteralExpression(node) => {
                    FormatJsRegexLiteralExpression.fmt_node(node, f)
                }
                AnyJsLiteralExpression::JsStringLiteralExpression(node) => {
                    FormatJsStringLiteralExpression.fmt_node(node, f)
                }
            },
            AnyJsExpression::JsArrayExpression(node) => {
                FormatJsArrayExpression::default().fmt_node(node, f)
            }
            AnyJsExpression::JsArrowFunctionExpression(node) => {
                FormatJsArrowFunctionExpression::default().fmt_node(node, f)
            }
            AnyJsExpression::JsAssignmentExpression(node) => {
                FormatJsAssignmentExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsAwaitExpression(node) => FormatJsAwaitExpression.fmt_node(node, f),
            AnyJsExpression::JsBinaryExpression(node) => FormatJsBinaryExpression.fmt_node(node, f),
            AnyJsExpression::JsBogusExpression(node) => FormatJsBogusExpression.fmt(node, f),
            AnyJsExpression::JsCallExpression(node) => FormatJsCallExpression.fmt_node(node, f),
            AnyJsExpression::JsClassExpression(node) => FormatJsClassExpression.fmt_node(node, f),
            AnyJsExpression::JsComputedMemberExpression(node) => {
                FormatJsComputedMemberExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsConditionalExpression(node) => {
                FormatJsConditionalExpression::default().fmt_node(node, f)
            }
            AnyJsExpression::JsFunctionExpression(node) => {
                FormatJsFunctionExpression::default().fmt_node(node, f)
            }
            AnyJsExpression::JsMetavariable(node) => FormatJsMetavariable.fmt_node(node, f),
            AnyJsExpression::JsIdentifierExpression(node) => {
                FormatJsIdentifierExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsImportCallExpression(node) => {
                FormatJsImportCallExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsImportMetaExpression(node) => {
                FormatJsImportMetaExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsInExpression(node) => FormatJsInExpression.fmt_node(node, f),
            AnyJsExpression::JsInstanceofExpression(node) => {
                FormatJsInstanceofExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsLogicalExpression(node) => {
                FormatJsLogicalExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsNewExpression(node) => FormatJsNewExpression.fmt_node(node, f),
            AnyJsExpression::JsNewTargetExpression(node) => {
                FormatJsNewTargetExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsObjectExpression(node) => FormatJsObjectExpression.fmt_node(node, f),
            AnyJsExpression::JsParenthesizedExpression(node) => {
                FormatJsParenthesizedExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsPostUpdateExpression(node) => {
                FormatJsPostUpdateExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsPreUpdateExpression(node) => {
                FormatJsPreUpdateExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsSequenceExpression(node) => {
                FormatJsSequenceExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsStaticMemberExpression(node) => {
                FormatJsStaticMemberExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsSuperExpression(node) => FormatJsSuperExpression.fmt_node(node, f),
            AnyJsExpression::JsTemplateExpression(node) => {
                FormatJsTemplateExpression.fmt_node(node, f)
            }
            AnyJsExpression::JsThisExpression(node) => FormatJsThisExpression.fmt_node(node, f),
            AnyJsExpression::JsUnaryExpression(node) => FormatJsUnaryExpression.fmt_node(node, f),
            AnyJsExpression::JsYieldExpression(node) => FormatJsYieldExpression.fmt_node(node, f),
            AnyJsExpression::JsxTagExpression(node) => FormatJsxTagExpression.fmt_node(node, f),
            AnyJsExpression::TsAsExpression(node) => FormatTsAsExpression.fmt_node(node, f),
            AnyJsExpression::TsInstantiationExpression(node) => {
                FormatTsInstantiationExpression.fmt_node(node, f)
            }
            AnyJsExpression::TsNonNullAssertionExpression(node) => {
                FormatTsNonNullAssertionExpression.fmt_node(node, f)
            }
            AnyJsExpression::TsSatisfiesExpression(node) => {
                FormatTsSatisfiesExpression.fmt_node(node, f)
            }
            AnyJsExpression::TsTypeAssertionExpression(node) => {
                FormatTsTypeAssertionExpression.fmt_node(node, f)
            }
        }
    }
}
