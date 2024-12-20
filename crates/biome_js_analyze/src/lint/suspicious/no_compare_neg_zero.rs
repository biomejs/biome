use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsSyntaxKind, JsUnaryOperator,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxToken};

use crate::JsRuleAction;

pub struct NoCompareNegZeroState {
    operator_kind: &'static str,
    left_need_replaced: bool,
    right_need_replaced: bool,
}

declare_lint_rule! {
    /// Disallow comparing against `-0`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// (1 >= -0)
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// (1 >= 0)
    ///```
    pub NoCompareNegZero {
        version: "1.0.0",
        name: "noCompareNegZero",
        language: "js",
        sources: &[RuleSource::Eslint("no-compare-neg-zero")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoCompareNegZero {
    type Query = Ast<JsBinaryExpression>;
    type State = NoCompareNegZeroState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if !node.is_comparison_operator() {
            return None;
        }

        let op = node.operator_token().ok()?;
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let is_left_neg_zero = is_neg_zero(&left).unwrap_or(false);
        let is_right_neg_zero = is_neg_zero(&right).unwrap_or(false);
        if is_left_neg_zero || is_right_neg_zero {
            // SAFETY: Because we know those T![>] | T![>=] | T![<] | T![<=] | T![==] | T![===] | T![!=] | T![!==] SyntaxKind will
            // always success in to_string, you could look at our test case `noCompareNegZero.js`
            let operator_kind = op.kind().to_string()?;

            Some(NoCompareNegZeroState {
                operator_kind,
                left_need_replaced: is_left_neg_zero,
                right_need_replaced: is_right_neg_zero,
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Do not use the "{state.operator_kind}" operator to compare against -0."
            },
        ))
    }
    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        if state.left_need_replaced {
            mutation.replace_node(
                node.left().ok()?,
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(SyntaxToken::new_detached(
                            JsSyntaxKind::JS_NUMBER_LITERAL,
                            "0",
                            [],
                            [],
                        )),
                    ),
                ),
            );
        }

        if state.right_need_replaced {
            mutation.replace_node(
                node.right().ok()?,
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(SyntaxToken::new_detached(
                            JsSyntaxKind::JS_NUMBER_LITERAL,
                            "0",
                            [],
                            [],
                        )),
                    ),
                ),
            );
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace -0 with 0" }.to_owned(),
            mutation,
        ))
    }
}

fn is_neg_zero(node: &AnyJsExpression) -> Option<bool> {
    match node {
        AnyJsExpression::JsUnaryExpression(expr) => {
            if !matches!(expr.operator().ok()?, JsUnaryOperator::Minus) {
                return Some(false);
            }
            let argument = expr.argument().ok()?;

            if let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(expr),
            ) = argument
            {
                Some(expr.value_token().ok()?.text_trimmed() == "0")
            } else {
                Some(false)
            }
        }
        _ => Some(false),
    }
}
