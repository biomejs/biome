use crate::utils::is_node_equal;
use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsLogicalExpression, JsLogicalOperator, JsUnaryOperator,
};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of yoda expressions.
    ///
    /// Yoda expressions can be confusing to some people, the rule forbids the use of it to improve code readability.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if ("red" == value) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (true === value) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (5 != value) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (value === "red") {}
    /// ```
    ///
    /// ```js
    /// if (value === value) {}
    /// ```
    ///
    /// ```js
    /// if (value != 5) {}
    /// ```
    pub NoYodaExpression {
        version: "next",
        name: "noYodaExpression",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoYodaExpression {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok();
        let right = node.right().ok();
        let operator = node.operator().ok();
        let parent_logical_expression = node.parent::<JsLogicalExpression>();
        let has_yoda_expression = is_comparison_operator(&operator)
            && is_literal_expression(&left)
            && !is_literal_expression(&right)
            && !is_range_assertion(&parent_logical_expression);

        has_yoda_expression.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the use of yoda expressions."
                },
            )
            .note(markup! {
                "Yoda expressions can be confusing to some people, invert the expression for better readability."
            }),
        )
    }
}

fn is_literal_expression(expression: &Option<AnyJsExpression>) -> bool {
    match expression {
        // Any literal (1, true, null, etc)
        Some(AnyJsExpression::AnyJsLiteralExpression(_)) => true,

        // Static template literals (`foo`)
        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => template_expression
            .elements()
            .into_iter()
            .all(|element| element.as_js_template_chunk_element().is_some()),

        // Negative numeric literal (-1)
        Some(AnyJsExpression::JsUnaryExpression(unary_expression)) => {
            let is_minus_operator =
                matches!(unary_expression.operator(), Ok(JsUnaryOperator::Minus));
            let is_number_expression = matches!(
                unary_expression.argument(),
                Ok(AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                ))
            );

            is_minus_operator && is_number_expression
        }
        _ => false,
    }
}

fn is_comparison_operator(operator: &Option<JsBinaryOperator>) -> bool {
    matches!(
        operator,
        Some(
            JsBinaryOperator::LessThan
                | JsBinaryOperator::GreaterThan
                | JsBinaryOperator::LessThanOrEqual
                | JsBinaryOperator::GreaterThanOrEqual
                | JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality
        )
    )
}

fn is_range_assertion(parent: &Option<JsLogicalExpression>) -> bool {
    if let Some(parent_logical_expression) = parent {
        return match (
            parent_logical_expression.left(),
            parent_logical_expression.right(),
            parent_logical_expression.operator(),
        ) {
            (
                Ok(AnyJsExpression::JsBinaryExpression(left)),
                Ok(AnyJsExpression::JsBinaryExpression(right)),
                Ok(operator),
            ) => {
                is_range_assertion_operator(left.operator().ok())
                    && is_range_assertion_operator(right.operator().ok())
                    && (is_inside_range_assertion(&operator, &left, &right)
                        || is_outside_range_assertion(&operator, &left, &right))
                    && !is_nested_logical_expression(parent_logical_expression)
            }
            _ => false,
        };
    }

    false
}

/// Determines whether the operator is one of those used in range assertions (< or <=)
fn is_range_assertion_operator(operator: Option<JsBinaryOperator>) -> bool {
    matches!(
        operator,
        Some(JsBinaryOperator::LessThan | JsBinaryOperator::LessThanOrEqual)
    )
}

/// Determines whether the nodes compose an inside range assertion, like `0 <= x && x < 1`
fn is_inside_range_assertion(
    operator: &JsLogicalOperator,
    left_binary_expression: &JsBinaryExpression,
    right_binary_expression: &JsBinaryExpression,
) -> bool {
    let left_identifier = left_binary_expression.right().ok();
    let right_identifier = right_binary_expression.left().ok();

    if !matches!(operator, JsLogicalOperator::LogicalAnd)
        || !is_same_identifier(&left_identifier, &right_identifier)
    {
        return false;
    }

    let left_literal = extract_string_value(left_binary_expression.left().as_ref().ok());
    let right_literal = extract_string_value(right_binary_expression.right().as_ref().ok());

    match (left_literal, right_literal) {
        (None, None) => false,
        (Some(_), None) => true,
        (None, Some(_)) => true,
        (Some(left_value), Some(right_value)) => left_value <= right_value,
    }
}

/// Determines whether the nodes compose an outside range assertion, like `x < 0 || 1 <= x`
fn is_outside_range_assertion(
    operator: &JsLogicalOperator,
    left_binary_expression: &JsBinaryExpression,
    right_binary_expression: &JsBinaryExpression,
) -> bool {
    let left_identifier = left_binary_expression.left().ok();
    let right_identifier = right_binary_expression.right().ok();

    if !matches!(operator, JsLogicalOperator::LogicalOr)
        || !is_same_identifier(&left_identifier, &right_identifier)
    {
        return false;
    }

    let left_literal = extract_string_value(left_binary_expression.right().as_ref().ok());
    let right_literal = extract_string_value(right_binary_expression.left().as_ref().ok());

    match (left_literal, right_literal) {
        (None, None) => false,
        (Some(_), None) => true,
        (None, Some(_)) => true,
        (Some(left_value), Some(right_value)) => left_value <= right_value,
    }
}

fn is_nested_logical_expression(logical_expression: &JsLogicalExpression) -> bool {
    logical_expression.parent::<JsLogicalExpression>().is_some()
}

fn is_same_identifier(
    left_expression: &Option<AnyJsExpression>,
    right_expression: &Option<AnyJsExpression>,
) -> bool {
    match (left_expression, right_expression) {
        (Some(left), Some(right)) => is_node_equal(left.syntax(), right.syntax()),
        _ => false,
    }
}

fn extract_string_value(expression: Option<&AnyJsExpression>) -> Option<String> {
    match expression {
        Some(AnyJsExpression::AnyJsLiteralExpression(literal_expression)) => Some(
            literal_expression
                .as_static_value()
                .map(|static_value| static_value.text().to_string())
                .unwrap_or(String::new()),
        ),

        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => Some(
            template_expression
                .elements()
                .into_iter()
                .fold(String::new(), |acc, element| {
                    acc + element
                        .as_js_template_chunk_element()
                        .unwrap()
                        .text()
                        .as_str()
                }),
        ),

        _ => None,
    }
}
