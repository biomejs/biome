use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{
    js_binary_expression, js_string_literal, js_string_literal_expression,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TextSize};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary concatenation of string or template literals.
    ///
    /// This rule aims to flag the concatenation of 2 literals when they could be combined into a single literal. Literals can be strings or template literals.
    /// Concatenation of multiple strings is allowed when the strings are spread over multiple lines in order to prevent exceeding the maximum line width.
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
    pub NoUselessStringConcat {
        version: "1.8.0",
        name: "noUselessStringConcat",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-concat")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessStringConcat {
    type Query = Ast<JsBinaryExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let parent_binary_expression = get_parent_binary_expression(node);

        // Prevent duplicated error reportings when the parent is a useless concatenation too, i.e.: "a" + "b" + "c"
        if parent_binary_expression.is_some()
            && get_useless_concat(&parent_binary_expression?).is_some()
        {
            return None;
        }

        get_useless_concat(node)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Useless string concatenation."
                },
            )
            .note(markup! {
                "Consider turning the expression into a single string to improve readability and runtime performance."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let left = node.left().ok();
        let right = node.right().ok();

        if is_numeric_calculation(&left) || is_numeric_calculation(&right) {
            return None;
        }

        let left_string = extract_string_value(&left);
        let right_string = extract_string_value(&right);

        let fix_result = match (left, left_string, right_string) {
            // Handle simple concatenations like "a" + "b"
            (_, Some(left_string_value), Some(right_string_value)) => {
                let concatenated_string = left_string_value + right_string_value.as_str();
                let string_literal_expression =
                    js_string_literal_expression(js_string_literal(concatenated_string.as_str()));

                mutation.replace_element(node.clone().into(), string_literal_expression.into());
                Some(())
            }

            // Handle nested concatenations like "a" + "b" + "c"
            (
                Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)),
                None,
                Some(right_string_value),
            ) => {
                let binary_expression =
                    concat_binary_expression(&left_binary_expression, right_string_value.as_str())?;

                mutation.replace_element(node.clone().into(), binary_expression.into());
                Some(())
            }

            // Handle concatenations where the left part is a parenthesized expression, like ("a" + "b") + "c"
            (
                Some(AnyJsExpression::JsParenthesizedExpression(left_parenthesized_expression)),
                _,
                Some(right_string),
            ) => match left_parenthesized_expression.expression() {
                Ok(AnyJsExpression::JsBinaryExpression(left_binary_expression)) => {
                    let binary_expression =
                        concat_binary_expression(&left_binary_expression, right_string.as_str())?;

                    mutation.replace_element(node.clone().into(), binary_expression.into());
                    Some(())
                }
                _ => None,
            },

            _ => None,
        };

        fix_result.and(Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the useless concatenation" }.to_owned(),
            mutation,
        )))
    }
}

fn get_useless_concat(node: &JsBinaryExpression) -> Option<TextRange> {
    if is_stylistic_concatenation(node) {
        return None;
    }

    is_concatenation(node)
}

fn is_string_expression(expression: &Option<AnyJsExpression>) -> bool {
    expression.as_ref().is_some_and(|node| {
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

fn is_numeric_expression(expression: &Option<AnyJsExpression>) -> bool {
    match expression.as_ref() {
        Some(AnyJsExpression::AnyJsLiteralExpression(literal_expression)) => matches!(
            literal_expression,
            AnyJsLiteralExpression::JsNumberLiteralExpression(_)
        ),

        _ => false,
    }
}

fn is_numeric_calculation(expression: &Option<AnyJsExpression>) -> bool {
    match expression.as_ref() {
        Some(AnyJsExpression::JsBinaryExpression(binary_expression)) => {
            is_numeric_expression(&binary_expression.left().ok())
                && is_numeric_expression(&binary_expression.right().ok())
        }
        _ => false,
    }
}

fn is_binary_expression_with_literal_string(expression: &Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = expression {
        // If the binary expression has an identifier expression inside, we can't statically ensure the type of the operation
        let has_left_identifier_expression = matches!(
            &binary_expression.left().ok(),
            Some(AnyJsExpression::JsIdentifierExpression(_))
        );
        let has_right_string_expression = is_string_expression(&binary_expression.right().ok());

        return !has_left_identifier_expression && has_right_string_expression;
    }

    false
}

fn is_concatenation(binary_expression: &JsBinaryExpression) -> Option<TextRange> {
    let left = binary_expression.left().ok();
    let right = binary_expression.right().ok();
    let has_left_string_expression = is_string_expression(&left)
        || is_binary_expression_with_literal_string(&left)
        || is_parenthesized_concatenation(&left);
    let has_right_string_expression = is_string_expression(&right)
        || is_binary_expression_with_literal_string(&right)
        || is_parenthesized_concatenation(&right);
    let has_left_numeric_expression = is_numeric_expression(&left) || is_numeric_calculation(&left);
    let has_right_numeric_expression =
        is_numeric_expression(&right) || is_numeric_calculation(&right);
    let operator = binary_expression.operator().ok();

    let has_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));
    let has_string_expression = match (
        has_left_string_expression,
        has_left_numeric_expression,
        has_right_string_expression,
        has_right_numeric_expression,
    ) {
        (true, _, true, _) => true,     // "a" + "b"
        (false, true, true, _) => true, // 1 + "a"
        (true, _, false, true) => true, // "a" + 1
        _ => false,
    };

    if has_plus_operator && has_string_expression {
        let range_start = if is_binary_expression_with_literal_string(&left) {
            match left {
                Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)) => {
                    extract_concat_range(&left_binary_expression)
                }
                _ => None,
            }
        } else {
            left.map(|left| left.range().start())
        };
        let range_end = right.map(|right| right.range().end());

        return match (range_start, range_end) {
            (Some(range_start), Some(range_end)) => Some(TextRange::new(range_start, range_end)),
            _ => None,
        };
    }

    None
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

fn is_parenthesized_concatenation(expression: &Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) = expression {
        return is_binary_expression_with_literal_string(
            &parenthesized_expression.expression().ok(),
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

        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal_expression),
        )) => number_literal_expression
            .as_number()
            .map(|number_value| number_value.to_string()),

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

        Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) => {
            extract_string_value(&parenthesized_expression.expression().ok())
        }

        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => {
            let is_useless_template_literal = template_expression
                .elements()
                .into_iter()
                .all(|element| element.as_js_template_chunk_element().is_some());

            if is_useless_template_literal {
                let concatenated_string = template_expression.elements().into_iter().fold(
                    String::new(),
                    |acc, element| {
                        if let Some(chunk) = element.as_js_template_chunk_element() {
                            return acc + chunk.to_trimmed_string().as_str();
                        }
                        acc
                    },
                );

                return Some(concatenated_string);
            }

            None
        }

        _ => None,
    }
}

fn extract_concat_range(binary_expression: &JsBinaryExpression) -> Option<TextSize> {
    match (
        binary_expression.left().ok(),
        binary_expression.right().ok(),
    ) {
        (Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)), Some(right)) => {
            extract_concat_range(&left_binary_expression).or(Some(right.range().start()))
        }
        (Some(left_expression), _) => Some(left_expression.range().start()),
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
) -> Option<JsBinaryExpression> {
    let current_right = left_binary_expression.right().ok();

    if is_string_expression(&current_right) {
        let value = extract_string_value(&left_binary_expression.right().ok())?;
        let concatenated_string = value + right_string_value;
        let string_literal_expression =
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                js_string_literal_expression(js_string_literal(&concatenated_string)),
            ));
        let left = left_binary_expression.left().ok()?;
        let operator = left_binary_expression.operator_token().ok()?;

        return match left {
            AnyJsExpression::JsBinaryExpression(binary_expression) => {
                let expression: AnyJsExpression = binary_expression.clone().into();

                // Only concatenating strings when we are sure that the evaluated expression will turn into a string
                if is_binary_expression_with_literal_string(&Some(expression.clone())) {
                    concat_binary_expression(&binary_expression, concatenated_string.as_str())
                } else {
                    Some(js_binary_expression(
                        expression.clone(),
                        operator,
                        string_literal_expression,
                    ))
                }
            }
            _ => Some(js_binary_expression(
                left.clone(),
                operator,
                string_literal_expression,
            )),
        };
    }

    Some(left_binary_expression.clone())
}
