use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{is_negation, AnyJsExpression, JsInExpression, JsInstanceofExpression};
use biome_rowan::{declare_node_union, AstNode, AstNodeExt, BatchMutationExt};

declare_lint_rule! {
    /// Disallow using unsafe negation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// !1 in [1,2];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /**test*/!/** test*/1 instanceof [1,2];
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// -1 in [1,2];
    /// ~1 in [1,2];
    /// typeof 1 in [1,2];
    /// void 1 in [1,2];
    /// delete 1 in [1,2];
    /// +1 instanceof [1,2];
    /// ```
    pub NoUnsafeNegation {
        version: "1.0.0",
        name: "noUnsafeNegation",
        language: "js",
        sources: &[RuleSource::Eslint("no-unsafe-negation")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUnsafeNegation {
    type Query = Ast<JsInOrInstanceOfExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                let left = expr.left().ok()?;

                is_negation(left.syntax()).and(Some(()))
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => {
                let left = expr.property().ok()?;

                is_negation(left.syntax()).and(Some(()))
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The negation operator is used unsafely on the left side of this binary expression."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // The action could be splitted to three steps
        // 1. Remove `!` operator of unary expression
        // 2. Wrap the expression with `()`, convert the expression to a `JsParenthesizedExpression`
        // 3. Replace the `JsParenthesizedExpression` to `JsUnaryExpression` by adding a `JsUnaryOperator::LogicalNot`
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                let left = expr.left().ok()?;
                let unary_expression = left.as_js_unary_expression()?;
                let argument = unary_expression.argument().ok()?;
                let next_expr = expr
                    .clone()
                    .replace_node_discard_trivia(left.clone(), argument)?;
                let next_parenthesis_expression = make::parenthesized(
                    biome_js_syntax::AnyJsExpression::JsInstanceofExpression(next_expr),
                );
                let next_unary_expression = make::js_unary_expression(
                    unary_expression.operator_token().ok()?,
                    AnyJsExpression::JsParenthesizedExpression(next_parenthesis_expression),
                );
                mutation.replace_node(
                    AnyJsExpression::from(expr.clone()),
                    AnyJsExpression::from(next_unary_expression),
                );
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => {
                let left = expr.property().ok()?;
                let unary_expression = left.as_any_js_expression()?.as_js_unary_expression()?;
                let argument = unary_expression.argument().ok()?;
                let next_expr = expr.clone().replace_node_discard_trivia(
                    left.clone(),
                    biome_js_syntax::AnyJsInProperty::AnyJsExpression(argument),
                )?;
                let next_parenthesis_expression = make::parenthesized(
                    biome_js_syntax::AnyJsExpression::JsInExpression(next_expr),
                );
                let next_unary_expression = make::js_unary_expression(
                    unary_expression.operator_token().ok()?,
                    AnyJsExpression::JsParenthesizedExpression(next_parenthesis_expression),
                );
                mutation.replace_node(
                    AnyJsExpression::from(expr.clone()),
                    AnyJsExpression::from(next_unary_expression),
                );
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the expression with a parenthesis" }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    /// Enum for [JsInstanceofExpression] and [JsInExpression]
    pub JsInOrInstanceOfExpression  = JsInstanceofExpression  | JsInExpression
}
