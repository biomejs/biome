use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{
    js_binary_expression, js_string_literal, js_string_literal_expression,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow unnecessary concatenation of literals or template literals.
    ///
    /// This rule aims to flag the concatenation of 2 literals when they could be combined into a single literal. Literals can be strings or template literals.
    /// Concatenation of multiple strings in different lines to prevent big line widths are allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b" + "c";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = foo + "a" + "b";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = (foo + "a") + ("b" + "c");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1 + 1;
    /// ```
    ///
    /// ```js
    /// const a = 1 * '2';
    /// ```
    ///
    /// ```js
    /// const a = 1 - 2;
    /// ```
    ///
    /// ```js
    /// const a = foo + bar;
    /// ```
    ///
    /// ```js
    /// const a = 'foo' + bar;
    /// ```
    ///
    /// ```js
    /// const a = 'foo' +
    ///           'bar'
    /// ```
    pub NoUselessConcat {
        version: "next",
        name: "noUselessConcat",
        sources: &[RuleSource::Eslint("no-useless-concat")],
        recommended: false,
    }
}

impl Rule for NoUselessConcat {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let parent_binary_expression = get_parent_binary_expression(node);

        // Prevent duplicated error reportings when the parent is an useless concatenation too, i.e.: "a" + "b" + "c"
        if parent_binary_expression.is_some()
            && has_useless_concat(&parent_binary_expression.unwrap())
        {
            return None;
        }

        has_useless_concat(node).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Useless string concatenation."
                },
            )
            .note(markup! {
                "Consider joining the strings as a single one to improve readability and runtime performance."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let left = node.left().ok();
        let right = node.right().ok();
        let left_string = extract_string_value(&left);
        let right_string = extract_string_value(&right);

        match (left, left_string, right_string) {
            // Handle simple concatenations like "a" + "b"
            (_, Some(left_string_value), Some(right_string_value)) => {
                let concatenated_string = left_string_value + right_string_value.as_str();
                let string_literal_expression =
                    js_string_literal_expression(js_string_literal(concatenated_string.as_str()));

                mutation.replace_element(node.clone().into(), string_literal_expression.into());

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Remove the useless concatenation" }.to_owned(),
                    mutation,
                })
            }
            // Handle nested concatenations like "a" + "b" + "c"
            (
                Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)),
                None,
                Some(right_string_value),
            ) => {
                let binary_expression =
                    concat_binary_expression(&left_binary_expression, right_string_value.as_str());

                mutation.replace_element(node.clone().into(), binary_expression.into());
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Remove the useless concatenation" }.to_owned(),
                    mutation,
                })
            }
            _ => None,
        }
    }
}

fn has_useless_concat(node: &JsBinaryExpression) -> bool {
    let has_left_string_expression = is_string_expression(node.left().ok())
        || is_binary_expression_with_literal_string(node.left().ok())
        || is_parenthesized_concatenation(node.left().ok());

    has_left_string_expression && is_concatenation(node) && !is_stylistic_concatenation(node)
}

fn is_string_expression(expression: Option<AnyJsExpression>) -> bool {
    expression.is_some_and(|node| {
        match (
            node.as_any_js_literal_expression(),
            node.as_js_template_expression(),
        ) {
            (Some(literal_expression), _) => literal_expression
                .as_js_string_literal_expression()
                .is_some(),
            (_, Some(_template_expression)) => true,
            _ => false,
        }
    })
}

fn is_binary_expression_with_literal_string(expression: Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = expression {
        return is_string_expression(binary_expression.right().ok());
    }

    false
}

fn is_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let operator = binary_expression.operator().ok();
    let is_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));
    let is_string_expression = is_string_expression(binary_expression.right().ok())
        || is_parenthesized_concatenation(binary_expression.right().ok());

    is_plus_operator && is_string_expression
}

/// Returns if the passed `JsBinaryExpression` has a multiline string concatenation
fn is_stylistic_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let operator = binary_expression.operator().ok();
    let is_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));
    let has_newline_in_right = binary_expression.right().is_ok_and(|right| {
        match (
            right.as_any_js_literal_expression(),
            right.as_js_template_expression(),
        ) {
            (Some(literal_expression), _) => literal_expression
                .as_js_string_literal_expression()
                .is_some_and(|string_literal_expression| {
                    string_literal_expression
                        .as_fields()
                        .value_token
                        .is_ok_and(|token| token.has_leading_newline())
                }),
            (_, Some(template_expression)) => template_expression
                .l_tick_token()
                .is_ok_and(|token| token.has_leading_newline()),
            _ => false,
        }
    });

    is_plus_operator && has_newline_in_right
}

fn is_parenthesized_concatenation(expression: Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) = expression {
        return is_binary_expression_with_literal_string(
            parenthesized_expression.expression().ok(),
        );
    }

    false
}

fn extract_string_value(expression: &Option<AnyJsExpression>) -> Option<String> {
    match expression {
        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string_literal_expression),
        )) => string_literal_expression
            .inner_string_text()
            .map(|token_text| token_text.to_string())
            .ok(),

        Some(AnyJsExpression::JsBinaryExpression(binary_expression)) => {
            match (
                extract_string_value(&binary_expression.left().ok()),
                extract_string_value(&binary_expression.right().ok()),
            ) {
                (Some(left_string), Some(right_string)) => {
                    Some(left_string + right_string.as_str())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn get_parent_binary_expression(node: &JsBinaryExpression) -> Option<JsBinaryExpression> {
    let mut current_node = node.parent();

    while current_node.is_some() {
        if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = current_node {
            return Some(binary_expression);
        } else if let Some(c) = current_node {
            current_node = c.parent()
        }
    }

    None
}

fn concat_binary_expression(
    left_binary_expression: &JsBinaryExpression,
    right_string_value: &str,
) -> JsBinaryExpression {
    let current_right = left_binary_expression.right().ok();

    if is_string_expression(current_right) {
        let value = extract_string_value(&left_binary_expression.right().ok()).unwrap();
        let concatenated_string = value + right_string_value;
        let string_literal_expression =
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                js_string_literal_expression(js_string_literal(&concatenated_string)),
            ));

        let left = left_binary_expression.left().unwrap();
        let operator = left_binary_expression.operator_token().unwrap();
        let binary_expression =
            js_binary_expression(left.clone(), operator, string_literal_expression);

        return match left {
            AnyJsExpression::JsBinaryExpression(binary_expression) => {
                concat_binary_expression(&binary_expression, concatenated_string.as_str())
            }
            _ => binary_expression,
        };
    }

    left_binary_expression.clone()
}
