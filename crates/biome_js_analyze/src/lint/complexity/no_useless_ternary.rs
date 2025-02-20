use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsConditionalExpression, JsSyntaxKind,
    OperatorPrecedence, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow ternary operators when simpler alternatives exist.
    ///
    /// It’s a common mistake in JavaScript to use a conditional expression to select between two
    /// boolean values instead of using the logical NOT (`!`) or double NOT (`!!`) to convert the test to a boolean.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = x ? true : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo === 1 ? false : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo + 1 ? false : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo + 1 ? true : false;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = x === 2 ? 'Yes' : 'No';
    /// ```
    ///
    /// ```js
    /// var a = x === 2 ? 'Yes' : false;
    /// ```
    ///
    /// ## Resources
    ///
    /// Logical NOT: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT
    ///
    pub NoUselessTernary {
        version: "1.5.0",
        name: "noUselessTernary",
        language: "js",
        sources: &[RuleSource::Eslint("no-unneeded-ternary")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let alternate = node.alternate().ok()?;
        let consequent = node.consequent().ok()?;

        if is_boolean_literal(&alternate) && is_boolean_literal(&consequent) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unnecessary use of boolean literals in conditional expression."
                },
            )
            .note(markup! {
                "Simplify your code by directly assigning the result without using a ternary operator."
            })
            .note(markup! {
                "If your goal is negation, you may use the logical NOT (!) or double NOT (!!) operator for clearer and concise code.\n Check for more details about "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT">"NOT"</Hyperlink>" operator."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let node_expression_kind = node.test().ok()?.syntax().kind();
        let alternate = node
            .alternate()
            .ok()?
            .as_any_js_literal_expression()?
            .as_js_boolean_literal_expression()?
            .value_token()
            .ok()?;
        let consequent = node
            .consequent()
            .ok()?
            .as_any_js_literal_expression()?
            .as_js_boolean_literal_expression()?
            .value_token()
            .ok()?;

        let new_node;
        let mut mutation = ctx.root().begin();

        if alternate.text_trimmed() == consequent.text_trimmed() {
            if node_expression_kind == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION {
                let res_boolean_value = if consequent.text_trimmed() == "true" {
                    T![true]
                } else {
                    T![false]
                };

                new_node = biome_js_syntax::AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsBooleanLiteralExpression(
                        make::js_boolean_literal_expression(make::token(res_boolean_value)),
                    ),
                );
            } else {
                return None;
            }
        } else if alternate.text_trimmed() == "true" {
            new_node = invert_expression(&node.test().ok()?)?;
        } else if is_boolean_expression(&node.test().ok()?)? {
            match node_expression_kind {
                JsSyntaxKind::JS_BINARY_EXPRESSION => {
                    let left = node.test().ok()?.as_js_binary_expression()?.left().ok()?;
                    let right = node.test().ok()?.as_js_binary_expression()?.right().ok()?;
                    let operator = node
                        .test()
                        .ok()?
                        .as_js_binary_expression()?
                        .operator_token()
                        .ok()?
                        .kind();

                    new_node = AnyJsExpression::from(make::js_binary_expression(
                        left,
                        make::token_decorated_with_space(operator),
                        right,
                    ));
                }
                JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                    let left = node
                        .test()
                        .ok()?
                        .as_js_instanceof_expression()?
                        .left()
                        .ok()?;
                    let right = node
                        .test()
                        .ok()?
                        .as_js_instanceof_expression()?
                        .right()
                        .ok()?;
                    new_node = make::js_instanceof_expression(
                        left,
                        make::token_decorated_with_space(T![instanceof]),
                        right,
                    )
                    .into();
                }
                JsSyntaxKind::JS_IN_EXPRESSION => {
                    let property = node.test().ok()?.as_js_in_expression()?.property().ok()?;
                    let object = node.test().ok()?.as_js_in_expression()?.object().ok()?;
                    new_node = make::js_in_expression(
                        property,
                        make::token_decorated_with_space(T![in]),
                        object,
                    )
                    .into();
                }
                JsSyntaxKind::JS_UNARY_EXPRESSION => {
                    let argument = node
                        .test()
                        .ok()?
                        .as_js_unary_expression()?
                        .argument()
                        .ok()?;
                    new_node = make::js_unary_expression(make::token(T![!]), argument).into();
                }
                _ => return None,
            }
        } else {
            let new_expression = invert_expression(&node.test().ok()?)?;
            new_node = make::js_unary_expression(make::token(T![!]), new_expression).into();
        }

        mutation.replace_element(node.clone().into(), new_node.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the conditional expression with" }.to_owned(),
            mutation,
        ))
    }
}

fn is_boolean_literal(expression: &AnyJsExpression) -> bool {
    let expr_kind = expression.syntax().kind();
    if expr_kind == JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION {
        return true;
    }
    false
}

fn is_boolean_expression(expression: &AnyJsExpression) -> Option<bool> {
    match expression.syntax().kind() {
        JsSyntaxKind::JS_BINARY_EXPRESSION => {
            if expression
                .as_js_binary_expression()?
                .is_comparison_operator()
            {
                return Some(true);
            }
        }
        JsSyntaxKind::JS_UNARY_EXPRESSION => {
            let operator = expression.as_js_unary_expression()?.operator_token().ok()?;
            if operator.kind() == JsSyntaxKind::BANG {
                return Some(true);
            }
        }
        JsSyntaxKind::JS_INSTANCEOF_EXPRESSION | JsSyntaxKind::JS_IN_EXPRESSION => {
            return Some(true);
        }
        _ => return Some(false),
    };

    Some(false)
}

fn invert_expression(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    if expression.syntax().kind() == JsSyntaxKind::JS_BINARY_EXPRESSION {
        let operator = expression
            .as_js_binary_expression()?
            .operator_token()
            .ok()?;
        let suggested_operator = match operator.kind() {
            JsSyntaxKind::EQ2 => Some(T![!=]),
            JsSyntaxKind::EQ3 => Some(T![!==]),
            JsSyntaxKind::NEQ => Some(T![==]),
            JsSyntaxKind::NEQ2 => Some(T![===]),
            _ => None,
        };

        if let Some(operator) = suggested_operator {
            let left = expression.as_js_binary_expression()?.left().ok()?;
            let right = expression.as_js_binary_expression()?.right().ok()?;
            let new_node = AnyJsExpression::from(make::js_binary_expression(
                left,
                make::token(operator),
                right,
            ));

            return Some(new_node);
        }
    }

    if expression.precedence().ok()? < OperatorPrecedence::Unary {
        let new_node = make::parenthesized(expression.clone()).into();
        let new_node = make::js_unary_expression(make::token(T![!]), new_node).into();
        return Some(new_node);
    }

    let new_node = make::js_unary_expression(make::token(T![!]), expression.clone()).into();

    Some(new_node)
}
