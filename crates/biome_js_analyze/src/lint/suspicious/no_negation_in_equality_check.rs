use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsSyntaxKind, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_negation_in_equality_check::NoNegationInEqualityCheckOptions;

declare_lint_rule! {
    /// Disallow the use of negation `!` in the left-hand side of an equality, as it can be confusing.
    ///
    /// The negation operator `!` has higher precedence than equality operators (`===`, `!==`).
    /// Consequently, an expression like `!a === b` is evaluated as `(!a) === b`, which compares the
    /// boolean result of `!a` with `b`. This is often a mistake, as the user likely intended
    /// `!(a === b)` or `a !== b`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// !a === b
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// !a !== b
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// !(a) === b
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a !== b
    /// ```
    ///
    /// ```js
    /// !(a === b)
    /// ```
    ///
    /// ```js
    /// (!!a) === b
    /// ```
    pub NoNegationInEqualityCheck {
        version: "next",
        name: "noNegationInEqualityCheck",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    negated_expression: AnyJsExpression,
    operator_kind: JsSyntaxKind,
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let n = ctx.query();
        let op_kind = n.operator_token().ok()?.kind();

        if !matches!(op_kind, JsSyntaxKind::EQ3 | JsSyntaxKind::NEQ2) {
            return None;
        }

        let left = n.left().ok()?;
        let unary_expr = left.as_js_unary_expression()?;

        if unary_expr.operator().ok()? == JsUnaryOperator::LogicalNot {
            // Skip double negation !!
            let argument = unary_expr.argument().ok()?;
            if let AnyJsExpression::JsUnaryExpression(inner_unary) = &argument
                && inner_unary.operator().ok()? == JsUnaryOperator::LogicalNot
            {
                return None;
            }

            return Some(RuleState {
                negated_expression: argument,
                operator_kind: op_kind,
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The negation operator "<Emphasis>"!"</Emphasis>" is evaluated before the equality operator."
                },
            )
            .note(markup! {
                "This is often a mistake as it compares a boolean value with another value."
            })
            .note(markup! {
                "If you want to negate the whole equality, use parentheses: "<Emphasis>"!(a === b)"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let new_operator = match state.operator_kind {
            JsSyntaxKind::EQ3 => T![!==],
            JsSyntaxKind::NEQ2 => T![===],
            _ => return None,
        };

        let old_op = node.operator_token().ok()?;
        let new_op_token = make::token(new_operator)
            .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
        mutation.replace_token(old_op, new_op_token);

        let left = node.left().ok()?;
        mutation.replace_node(left, state.negated_expression.clone());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Exchange the operator to remove the negation." }.to_owned(),
            mutation,
        ))
    }
}
