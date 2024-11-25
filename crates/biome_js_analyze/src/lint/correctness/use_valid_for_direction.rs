use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentOperator, JsBinaryOperator, JsForStatement,
    JsIdentifierAssignment, JsIdentifierExpression, JsPostUpdateOperator, JsUnaryOperator,
};

declare_lint_rule! {
    /// Enforce "for" loop update clause moving the counter in the right direction.
    ///
    /// A for loop with a stop condition that can never be reached,
    /// such as one with a counter that moves in the wrong direction, will run infinitely.
    /// While there are occasions when an infinite loop is intended, the convention is to construct such loops as while loops.
    /// More typically, an infinite for loop is a bug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 0; i < 10; i--) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 10; i >= 0; i++) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 0; i > 10; i++) {
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (var i = 0; i < 10; i++) {
    /// }
    /// ```
    pub UseValidForDirection {
        version: "1.0.0",
        name: "useValidForDirection",
        language: "js",
        sources: &[RuleSource::Eslint("for-direction")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseValidForDirection {
    type Query = Ast<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let test = node.test()?;
        let binary_expr = test.as_js_binary_expression()?;
        let operator = binary_expr.operator().ok()?;

        let is_less_than = matches!(
            operator,
            JsBinaryOperator::LessThan | JsBinaryOperator::LessThanOrEqual
        );
        let is_greater_than = matches!(
            operator,
            JsBinaryOperator::GreaterThan | JsBinaryOperator::GreaterThanOrEqual
        );

        if !is_less_than && !is_greater_than {
            return None;
        }

        match node.update()? {
            AnyJsExpression::JsPostUpdateExpression(update_expr) => {
                let binary_expr_left = binary_expr.left().ok()?;
                let counter_ident = binary_expr_left.as_js_identifier_expression()?;
                let update_expr_operand = update_expr.operand().ok()?;
                let update_ident = update_expr_operand.as_js_identifier_assignment()?;

                if !is_identifier_same(counter_ident, update_ident)? {
                    return None;
                }

                if update_expr.operator().ok()? == JsPostUpdateOperator::Increment
                    && is_greater_than
                {
                    return Some(());
                }

                if update_expr.operator().ok()? == JsPostUpdateOperator::Decrement && is_less_than {
                    return Some(());
                }
            }
            AnyJsExpression::JsAssignmentExpression(assignment_expr) => {
                let binary_expr_left = binary_expr.left().ok()?;
                let counter_ident = binary_expr_left.as_js_identifier_expression()?;
                let assignment_expr_left = assignment_expr.left().ok()?;
                let update_ident = assignment_expr_left
                    .as_any_js_assignment()?
                    .as_js_identifier_assignment()?;

                if !is_identifier_same(counter_ident, update_ident)? {
                    return None;
                }

                if assignment_expr
                    .right()
                    .ok()?
                    .as_js_identifier_expression()
                    .is_some()
                {
                    return None;
                }

                match assignment_expr.operator().ok()? {
                    JsAssignmentOperator::AddAssign => {
                        if is_greater_than {
                            return Some(());
                        }

                        let assignment_expr_right = assignment_expr.right().ok()?;
                        let unary_expr = assignment_expr_right.as_js_unary_expression()?;
                        if is_less_than && unary_expr.operator().ok()? == JsUnaryOperator::Minus {
                            return Some(());
                        }
                    }
                    JsAssignmentOperator::SubtractAssign => {
                        if is_less_than {
                            return Some(());
                        }

                        let assignment_expr_right = assignment_expr.right().ok()?;
                        let unary_expr = assignment_expr_right.as_js_unary_expression()?;
                        if is_greater_than && unary_expr.operator().ok()? == JsUnaryOperator::Minus
                        {
                            return Some(());
                        }
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let l_paren_range = node.l_paren_token().ok()?.text_trimmed_range();
        let r_paren_range = node.r_paren_token().ok()?.text_trimmed_range();
        Some(RuleDiagnostic::new(
            rule_category!(),
            l_paren_range.cover(r_paren_range),
            markup! {
                "The update clause in this loop moves the variable in the wrong direction."
            },
        ))
    }
}

fn is_identifier_same(
    counter_ident: &JsIdentifierExpression,
    update_ident: &JsIdentifierAssignment,
) -> Option<bool> {
    Some(
        counter_ident
            .name()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
            == update_ident.name_token().ok()?.text_trimmed(),
    )
}
