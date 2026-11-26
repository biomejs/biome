use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssValue, AnyScssExpression, AnyScssExpressionItem, ScssBinaryExpression, ScssIfAtRule, T,
};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_scss_null_comparison::NoScssNullComparisonOptions;

declare_lint_rule! {
    /// Disallow comparing values to `null` in Sass `@if` conditions.
    ///
    /// In Sass, `null` counts as false in a condition. Writing `$value == null` or `$value != null`
    /// in an `@if` condition is longer than needed: `not $value` and `$value` check the same thing
    /// in most cases.
    ///
    /// The rule checks the conditions of `@if` and `@else if`, including comparisons combined
    /// with `and`, `or` and `not`.
    ///
    /// A `!= null` check followed by `and`, as in `$value != null and $value > 1`, isn't reported.
    /// This is a common way to check that a value exists before using it in the rest of the
    /// condition.
    ///
    /// :::note
    /// `false` also counts as false in a condition. If a value can be `false` and your code needs
    /// to treat `false` differently from `null`, keep the comparison and suppress the rule for that
    /// line.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```scss,expect_diagnostic
    /// a {
    ///   @if $value == null {}
    /// }
    /// ```
    ///
    /// ```scss,expect_diagnostic
    /// a {
    ///   @if $value != null {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```scss
    /// a {
    ///   @if $value {}
    ///   @if not $value {}
    ///   @if $value != null and $value > 1 {}
    /// }
    /// ```
    ///
    pub NoScssNullComparison {
        version: "2.6.7",
        name: "noScssNullComparison",
        language: "css",
        recommended: true,
        sources: &[RuleSource::StylelintScss("at-if-no-null").inspired()],
    }
}

impl Rule for NoScssNullComparison {
    type Query = Ast<ScssIfAtRule>;
    type State = ScssBinaryExpression;
    type Signals = Box<[Self::State]>;
    type Options = NoScssNullComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Ok(condition) = ctx.query().condition() else {
            return Box::default();
        };

        let mut comparisons = Vec::new();
        // Each entry pairs an expression whose value is used as a condition with whether the
        // expression is the left side of an `and`.
        let mut stack = vec![(AnyScssExpression::ScssExpression(condition), false)];
        while let Some((expression, is_and_guard)) = stack.pop() {
            match expression {
                AnyScssExpression::ScssExpression(expression) => {
                    // An expression with several items is a list, which isn't a condition.
                    let items = expression.items();
                    if items.len() != 1 {
                        continue;
                    }
                    let inner = match items.first() {
                        Some(AnyScssExpressionItem::ScssBinaryExpression(inner)) => {
                            AnyScssExpression::ScssBinaryExpression(inner)
                        }
                        Some(AnyScssExpressionItem::ScssUnaryExpression(inner)) => {
                            AnyScssExpression::ScssUnaryExpression(inner)
                        }
                        Some(AnyScssExpressionItem::ScssParenthesizedExpression(inner)) => {
                            AnyScssExpression::ScssParenthesizedExpression(inner)
                        }
                        _ => continue,
                    };
                    stack.push((inner, is_and_guard));
                }
                AnyScssExpression::ScssParenthesizedExpression(expression) => {
                    if let Ok(inner) = expression.expression() {
                        stack.push((inner, is_and_guard));
                    }
                }
                AnyScssExpression::ScssUnaryExpression(expression) => {
                    if expression.operator().is_ok_and(|operator| operator.kind() == T![not])
                        && let Ok(inner) = expression.expression()
                    {
                        stack.push((inner, false));
                    }
                }
                AnyScssExpression::ScssBinaryExpression(expression) => {
                    let Ok(operator) = expression.operator() else {
                        continue;
                    };
                    match operator.kind() {
                        T![and] => {
                            if let Ok(right) = expression.right() {
                                stack.push((right, is_and_guard));
                            }
                            if let Ok(left) = expression.left() {
                                stack.push((left, true));
                            }
                        }
                        T![or] => {
                            if let Ok(right) = expression.right() {
                                stack.push((right, false));
                            }
                            if let Ok(left) = expression.left() {
                                stack.push((left, false));
                            }
                        }
                        T![==] => {
                            if is_null_comparison(&expression) {
                                comparisons.push(expression);
                            }
                        }
                        T![!=] => {
                            if !is_and_guard && is_null_comparison(&expression) {
                                comparisons.push(expression);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        comparisons.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, comparison: &Self::State) -> Option<RuleDiagnostic> {
        let is_equality = comparison.operator().ok()?.kind() == T![==];
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                comparison.range(),
                markup! {
                    "This condition compares a value to "<Emphasis>"null"</Emphasis>"."
                },
            )
            .note(markup! {
                "In Sass, "<Emphasis>"null"</Emphasis>" counts as false in a condition, so the comparison isn't needed."
            })
            .note(if is_equality {
                markup! {
                    "Check the value with "<Emphasis>"not"</Emphasis>" instead, as in "<Emphasis>"@if not $value"</Emphasis>"."
                }
            } else {
                markup! {
                    "Use the value as the condition instead, as in "<Emphasis>"@if $value"</Emphasis>"."
                }
            }),
        )
    }
}

/// Returns whether either side of `comparison` is the `null` keyword.
fn is_null_comparison(comparison: &ScssBinaryExpression) -> bool {
    [comparison.left(), comparison.right()]
        .into_iter()
        .flatten()
        .any(|operand| is_null(&operand))
}

fn is_null(expression: &AnyScssExpression) -> bool {
    let AnyScssExpression::AnyCssValue(AnyCssValue::CssIdentifier(identifier)) = expression else {
        return false;
    };
    identifier
        .value_token()
        .is_ok_and(|token| token.text_trimmed() == "null")
}
