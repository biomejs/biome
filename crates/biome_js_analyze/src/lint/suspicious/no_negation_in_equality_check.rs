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
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Clone)]
pub enum RuleState {
    FlipOperator {
        negated_expression: AnyJsExpression,
        operator_kind: JsSyntaxKind,
    },
    NegateEntireEquality,
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let n = ctx.query();
        let op_kind = match n.operator_token().ok() {
            Some(op) => op.kind(),
            None => return vec![],
        };

        if !matches!(op_kind, JsSyntaxKind::EQ3 | JsSyntaxKind::NEQ2) {
            return vec![];
        }

        let left = match n.left().ok() {
            Some(left) => left,
            None => return vec![],
        };
        let unary_expr = match left.as_js_unary_expression() {
            Some(unary) => unary,
            None => return vec![],
        };

        if unary_expr.operator().ok() == Some(JsUnaryOperator::LogicalNot) {
            // Skip double negation !!
            let argument = match unary_expr.argument().ok() {
                Some(arg) => arg,
                None => return vec![],
            };
            if let AnyJsExpression::JsUnaryExpression(inner_unary) = &argument {
                if inner_unary.operator().ok() == Some(JsUnaryOperator::LogicalNot) {
                    return vec![];
                }
            }

            return vec![
                RuleState::FlipOperator {
                    negated_expression: argument,
                    operator_kind: op_kind,
                },
                RuleState::NegateEntireEquality,
            ];
        }

        vec![]
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diag = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The negation operator "<Emphasis>"!"</Emphasis>" is evaluated before the equality operator."
            },
        )
        .note(markup! {
            "This is often a mistake as it compares a boolean value with another value."
        });

        match state {
            RuleState::FlipOperator { .. } => Some(diag.note(markup! {
                "If you want to remove the negation, exchange the operator: "<Emphasis>"a !== b"</Emphasis>"."
            })),
            RuleState::NegateEntireEquality => Some(diag.note(markup! {
                "If you want to negate the whole equality, use parentheses: "<Emphasis>"!(a === b)"</Emphasis>"."
            })),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match state {
            RuleState::FlipOperator {
                negated_expression,
                operator_kind,
            } => {
                let new_operator = match operator_kind {
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
                mutation.replace_node(left, negated_expression.clone());

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Exchange the operator to remove the negation." }.to_owned(),
                    mutation,
                ))
            }
            RuleState::NegateEntireEquality => {
                let left = node.left().ok()?;
                let unary_expr = left.as_js_unary_expression()?;
                let argument = unary_expr.argument().ok()?;
                let op_token = unary_expr.operator_token().ok()?;

                let new_bin_expr = node.clone().with_left(argument);
                let paren_expr = make::js_parenthesized_expression(
                    make::token(T!['(']),
                    AnyJsExpression::JsBinaryExpression(new_bin_expr),
                    make::token(T![')']),
                );
                let new_unary_expr = make::js_unary_expression(
                    op_token,
                    AnyJsExpression::JsParenthesizedExpression(paren_expr),
                );

                mutation.replace_node(
                    AnyJsExpression::JsBinaryExpression(node.clone()),
                    AnyJsExpression::from(new_unary_expr),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Negate the whole equality with parentheses." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}
