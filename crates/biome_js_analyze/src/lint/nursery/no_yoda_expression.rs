use crate::{utils::is_node_equal, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{self, js_binary_expression, token};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsStatement, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsIfStatement, JsLanguage, JsLogicalExpression, JsLogicalOperator,
    JsParenthesizedExpression, JsSyntaxKind, JsUnaryOperator, JsWhileStatement, JsYieldArgument,
    JsYieldExpression, T,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxTriviaPiece, TriviaPieceKind, WalkEvent};

declare_rule! {
    /// Disallow the use of yoda expressions.
    ///
    /// A Yoda expression is a programming style where the two parts of the expression are reversed from the typical order in a conditional statement.
    /// A Yoda expression places the constant portion of the expression on the left side of the conditional statement. They can be confusing to some people,
    /// the rule forbids the use of it to improve code readability.
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
        sources: &[RuleSource::Eslint("yoda")],
        recommended: false,
        fix_kind: FixKind::Safe,
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
                "Yoda expressions can be confusing to some people, invert the expression operands for better readability."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let parent_statement = node.parent::<AnyJsStatement>();
        let parent_expression = node.parent::<AnyJsExpression>();
        let parent_yield_argument = node.parent::<JsYieldArgument>();
        let mut mutation = node.clone().begin();

        match (
            node.left(),
            node.right(),
            node.operator_token(),
            flip_operator(node.operator().ok()),
        ) {
            (Ok(left), Ok(right), Ok(operator_token), Some(flipped_operator)) => {
                let left_leading_trivia = extract_leading_trivia(&left);
                let left_trailing_trivia = extract_trailing_trivia(&left);
                let right_leading_trivia = extract_leading_trivia(&right);
                let right_trailing_trivia = extract_trailing_trivia(&right);
                let operator_leading_trivia = operator_token.leading_trivia().pieces();
                let operator_trailing_trivia = operator_token.trailing_trivia().pieces();
                let whitespace = make::token(T!(==))
                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                    .trailing_trivia()
                    .last();

                let has_missing_left_trivia = match (&left_leading_trivia, &parent_yield_argument) {
                    (trivia, Some(parent_yield_argument)) => {
                        let parent_yield_expression =
                            parent_yield_argument.parent::<JsYieldExpression>();
                        let has_trivia_on_parent_expression =
                            parent_yield_expression.clone().is_some_and(|expression| {
                                expression.yield_token().is_ok_and(|yield_token| {
                                    !yield_token.trailing_trivia().is_empty()
                                })
                            });

                        trivia.is_empty() && !has_trivia_on_parent_expression
                    }
                    _ => false,
                };
                let has_missing_right_trivia = match (
                    right.clone().syntax().last_trailing_trivia(),
                    parent_statement.clone(),
                    parent_expression.clone(),
                ) {
                    (_, Some(AnyJsStatement::JsIfStatement(_)), _) => false,
                    (_, Some(AnyJsStatement::JsWhileStatement(_)), _) => false,
                    (_, _, Some(AnyJsExpression::JsParenthesizedExpression(_))) => false,
                    (Some(trivia), _, _) => trivia.is_empty(),
                    (None, _, _) => true,
                };

                let new_left = if has_missing_left_trivia {
                    clone_with_trivia(&right, &left_leading_trivia, &left_trailing_trivia)
                        .with_leading_trivia_pieces(whitespace.clone())
                        .unwrap()
                } else {
                    clone_with_trivia(&right, &left_leading_trivia, &left_trailing_trivia)
                };
                let new_operator = token(flipped_operator)
                    .prepend_trivia_pieces(operator_leading_trivia)
                    .append_trivia_pieces(operator_trailing_trivia);
                let new_right = if has_missing_right_trivia {
                    clone_with_trivia(&left, &right_leading_trivia, &right_trailing_trivia)
                        .append_trivia_pieces(whitespace.clone())
                        .unwrap()
                } else {
                    clone_with_trivia(&left, &right_leading_trivia, &right_trailing_trivia)
                };

                let binary_expression = js_binary_expression(new_left, new_operator, new_right);

                if has_missing_right_trivia {
                    mutation.replace_element_discard_trivia(
                        node.to_owned().into_syntax().into(),
                        binary_expression.into_syntax().into(),
                    );
                } else {
                    mutation.replace_node_discard_trivia(node.clone(), binary_expression);
                }

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Flip the operators of the expression." }.to_owned(),
                    mutation,
                })
            }
            _ => None,
        }
    }
}

fn extract_leading_trivia(node: &AnyJsExpression) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    node.syntax()
        .first_leading_trivia()
        .map(|first_leading_trivia| first_leading_trivia.pieces().collect::<Vec<_>>())
        .unwrap_or_default()
}

fn extract_trailing_trivia(node: &AnyJsExpression) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    node.syntax()
        .last_trailing_trivia()
        .map(|last_trailing_trivia| last_trailing_trivia.pieces().collect::<Vec<_>>())
        .unwrap_or_default()
}

fn clone_with_trivia(
    node: &AnyJsExpression,
    leading_trivia: &[SyntaxTriviaPiece<JsLanguage>],
    trailing_trivia: &[SyntaxTriviaPiece<JsLanguage>],
) -> AnyJsExpression {
    node.clone()
        .with_leading_trivia_pieces(leading_trivia.to_owned())
        .unwrap()
        .with_trailing_trivia_pieces(trailing_trivia.to_owned())
        .unwrap()
}

fn is_literal_expression(expression: &Option<AnyJsExpression>) -> bool {
    match expression {
        // Any literal: 1, true, null, etc
        Some(AnyJsExpression::AnyJsLiteralExpression(_)) => true,

        // Static template literals: `foo`
        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => template_expression
            .elements()
            .into_iter()
            .all(|element| element.as_js_template_chunk_element().is_some()),

        // Negative numeric literal: -1
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

        // Parenthesized expression: (1)
        Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) => {
            is_literal_expression(&parenthesized_expression.expression().ok())
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
                    && is_wrapped_in_parenthesis(parent_logical_expression)
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
        (Some(left_value), Some(right_value)) => {
            compare_literals(left_value.as_str(), right_value.as_str())
        }
    }
}

/// Compare literals as number if possible, compare as string otherwise
fn compare_literals(left_string_value: &str, right_string_value: &str) -> bool {
    match (
        left_string_value.parse::<f64>(),
        right_string_value.parse::<f64>(),
    ) {
        (Ok(left_number_value), Ok(right_number_value)) => left_number_value <= right_number_value,
        _ => left_string_value <= right_string_value,
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
        (Some(left_value), Some(right_value)) => {
            compare_literals(left_value.as_str(), right_value.as_str())
        }
    }
}

fn is_wrapped_in_parenthesis(logical_expression: &JsLogicalExpression) -> bool {
    !matches!(
        (
            logical_expression.parent::<JsParenthesizedExpression>(),
            logical_expression.parent::<JsIfStatement>(),
            logical_expression.parent::<JsWhileStatement>(),
        ),
        (None, None, None)
    )
}

fn is_same_identifier(
    left_expression: &Option<AnyJsExpression>,
    right_expression: &Option<AnyJsExpression>,
) -> bool {
    match (left_expression, right_expression) {
        (Some(left), Some(right)) => {
            // We can't consider two call expressions equal here because the result of the expression might be different if we call it twice
            let has_call_expression = |expression: &AnyJsExpression| {
                expression.syntax().preorder().any(|event| match event {
                    WalkEvent::Leave(node) => JsCallExpression::can_cast(node.kind()),
                    _ => false,
                })
            };

            is_node_equal(left.syntax(), right.syntax())
                && !has_call_expression(left)
                && !has_call_expression(right)
        }
        _ => false,
    }
}

fn extract_string_value(expression: Option<&AnyJsExpression>) -> Option<String> {
    match expression {
        // "a", 1, null, undefined, etc
        Some(AnyJsExpression::AnyJsLiteralExpression(literal_expression)) => Some(
            literal_expression
                .as_static_value()
                .map_or(String::new(), |static_value| {
                    static_value.text().to_string()
                }),
        ),

        // `a`
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

        // -1
        Some(AnyJsExpression::JsUnaryExpression(unary_expression)) => {
            match (
                unary_expression.operator_token().ok(),
                unary_expression.argument().ok(),
            ) {
                (Some(operator_token), Some(argument)) => Some(
                    operator_token.to_string()
                        + &extract_string_value(Some(&argument))?.to_string(),
                ),
                _ => None,
            }
        }

        _ => None,
    }
}

fn flip_operator(operator: Option<JsBinaryOperator>) -> Option<JsSyntaxKind> {
    match operator {
        // === to ===
        Some(JsBinaryOperator::StrictEquality) => Some(T!(===)),

        // !== to !==
        Some(JsBinaryOperator::StrictInequality) => Some(T!(!==)),

        // == to ==
        Some(JsBinaryOperator::Equality) => Some(T!(==)),

        // != to !=
        Some(JsBinaryOperator::Inequality) => Some(T!(!=)),

        // < to >
        Some(JsBinaryOperator::LessThan) => Some(T!(>)),

        // > to <
        Some(JsBinaryOperator::GreaterThan) => Some(T!(<)),

        // <= to >=
        Some(JsBinaryOperator::LessThanOrEqual) => Some(T!(>=)),

        // >= to <=
        Some(JsBinaryOperator::GreaterThanOrEqual) => Some(T!(<=)),

        _ => None,
    }
}
