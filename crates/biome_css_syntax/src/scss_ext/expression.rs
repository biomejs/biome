use crate::{
    AnyCssExpression, AnyScssExpression, AnyScssExpressionItem, CssLanguage, CssSyntaxNode,
    CssSyntaxToken, ScssExpression, ScssIfAtRule, ScssKeywordArgument, ScssParenthesizedExpression,
    ScssWhileAtRule,
};
use biome_rowan::{AstNode, AstNodeList};
use std::iter;

impl AnyScssExpression {
    /// Returns the innermost expression beneath grouping parentheses.
    ///
    /// Handles single-item [`ScssExpression`] wrappers while retaining the final
    /// expression container, list, or map. Stops if the inner expression is missing.
    pub fn omit_parentheses(self) -> Self {
        let inner_expression = |expression: &Self| {
            expression
                .as_scss_parenthesized_expression()
                .cloned()
                .or_else(|| {
                    unwrap_single_expression_item(expression)
                        .and_then(|item| item.as_scss_parenthesized_expression().cloned())
                })
                .and_then(|node| node.expression().ok())
        };

        iter::successors(inner_expression(&self), inner_expression)
            .last()
            .unwrap_or(self)
    }
}

/// Returns `$arg: value` from a CSS expression that wraps an SCSS expression.
pub fn scss_keyword_argument_from_css_expression(
    value: &AnyCssExpression,
) -> Option<ScssKeywordArgument> {
    value
        .as_scss_expression()
        .cloned()
        .map(AnyScssExpression::ScssExpression)
        .and_then(|expression| scss_keyword_argument_from_expression(&expression))
}

/// Returns `$arg: value` when a syntax node is an SCSS expression wrapper.
pub fn scss_keyword_argument_from_syntax(node: &CssSyntaxNode) -> Option<ScssKeywordArgument> {
    AnyScssExpression::cast(node.clone())
        .and_then(|expression| scss_keyword_argument_from_expression(&expression))
}

/// Returns `$arg: value` when an SCSS expression only wraps that item.
pub fn scss_keyword_argument_from_expression(
    value: &AnyScssExpression,
) -> Option<ScssKeywordArgument> {
    unwrap_single_expression_item(value).and_then(|item| item.as_scss_keyword_argument().cloned())
}

/// Unwraps `ScssExpression` containers that only wrap a single item.
pub fn unwrap_single_expression_item(value: &AnyScssExpression) -> Option<AnyScssExpressionItem> {
    value.as_scss_expression().and_then(single_expression_item)
}

/// Returns the only item in an SCSS expression.
///
/// This returns `None` for expressions widened by separators or operators,
/// such as `a, b` or `a + b`.
pub fn single_expression_item(expression: &ScssExpression) -> Option<AnyScssExpressionItem> {
    let mut items = expression.items().iter();
    let first = items.next()?;

    items.next().is_none().then_some(first)
}

/// Returns `true` for SCSS comparison operators.
///
/// Examples: `1 > $width`, `$a == $b`, and `$a != null`.
pub fn is_scss_comparison_operator(operator: &CssSyntaxToken) -> bool {
    matches!(
        operator.kind(),
        T![>] | T![>=] | T![<] | T![<=] | T![==] | T![!=]
    )
}

/// Returns `true` for interpolation expression operands.
///
/// This matches direct `#{$name}` and a single-item wrapper, as in
/// `#{$name}/2px`.
fn is_scss_interpolation_expression(expression: &AnyScssExpression) -> bool {
    matches!(expression, AnyScssExpression::ScssInterpolation(_))
        || unwrap_single_expression_item(expression)
            .is_some_and(|item| matches!(item, AnyScssExpressionItem::ScssInterpolation(_)))
}

/// Returns `true` when the expression's left edge is interpolation.
///
/// Examples: `#{$a}/1` and `#{$a}/#{$b}`.
pub fn is_scss_expression_starting_with_interpolation(expression: &AnyScssExpression) -> bool {
    if is_scss_interpolation_expression(expression) {
        return true;
    }

    match expression {
        AnyScssExpression::ScssExpression(expression) => expression
            .items()
            .iter()
            .next()
            .is_some_and(|item| is_scss_expression_item_starting_with_interpolation(&item)),
        AnyScssExpression::ScssBinaryExpression(binary) => binary
            .left()
            .is_ok_and(|left| is_scss_expression_starting_with_interpolation(&left)),
        _ => false,
    }
}

/// Returns `true` when the expression's right edge is interpolation.
///
/// Examples: `1/#{$a}` and `-#{$a}`.
pub fn is_scss_expression_ending_with_interpolation(expression: &AnyScssExpression) -> bool {
    if is_scss_interpolation_expression(expression) {
        return true;
    }

    match expression {
        AnyScssExpression::ScssExpression(expression) => expression
            .items()
            .iter()
            .next_back()
            .is_some_and(|item| is_scss_expression_item_ending_with_interpolation(&item)),
        AnyScssExpression::ScssBinaryExpression(binary) => binary
            .right()
            .is_ok_and(|right| is_scss_expression_ending_with_interpolation(&right)),
        AnyScssExpression::ScssUnaryExpression(unary) => unary
            .expression()
            .is_ok_and(|expression| is_scss_expression_ending_with_interpolation(&expression)),
        _ => false,
    }
}

/// Returns `true` for parenthesized expression operands.
///
/// This matches direct `($name)` and a single-item wrapper, as in `$a * ($b)`.
pub fn is_scss_parenthesized_expression(expression: &AnyScssExpression) -> bool {
    matches!(
        expression,
        AnyScssExpression::ScssParenthesizedExpression(_)
    ) || unwrap_single_expression_item(expression)
        .is_some_and(|item| matches!(item, AnyScssExpressionItem::ScssParenthesizedExpression(_)))
}

/// Returns `true` for nodes inside SCSS parentheses.
///
/// Example: in `($a + $b)`, the binary expression is inside `(...)`.
pub fn is_in_scss_parenthesized_expression<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| ScssParenthesizedExpression::can_cast(ancestor.kind()))
}

fn is_scss_expression_item_starting_with_interpolation(item: &AnyScssExpressionItem) -> bool {
    match item {
        AnyScssExpressionItem::ScssInterpolation(_) => true,
        AnyScssExpressionItem::ScssBinaryExpression(binary) => binary
            .left()
            .is_ok_and(|left| is_scss_expression_starting_with_interpolation(&left)),
        _ => false,
    }
}

fn is_scss_expression_item_ending_with_interpolation(item: &AnyScssExpressionItem) -> bool {
    match item {
        AnyScssExpressionItem::ScssInterpolation(_) => true,
        AnyScssExpressionItem::ScssBinaryExpression(binary) => binary
            .right()
            .is_ok_and(|right| is_scss_expression_ending_with_interpolation(&right)),
        AnyScssExpressionItem::ScssUnaryExpression(unary) => unary
            .expression()
            .is_ok_and(|expression| is_scss_expression_ending_with_interpolation(&expression)),
        _ => false,
    }
}

/// Detects expressions owned by a top-level control condition.
///
/// `@if $a == 0 {}` and `@if ($a == 0) {}` match `$a == 0`.
/// `@if foo($a == 0) {}` does not because `$a == 0` belongs to `foo(...)`.
pub fn is_in_scss_control_condition_sequence<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    node.syntax()
        .ancestors()
        .find_map(ScssExpression::cast)
        .is_some_and(|expression| {
            is_direct_control_condition(&expression)
                || is_parenthesized_control_condition_expression(&expression)
        })
}

/// Checks that `$a == 0` is the direct condition in `@if $a == 0 {}`.
fn is_direct_control_condition(expression: &ScssExpression) -> bool {
    scss_control_condition(expression)
        .is_some_and(|condition| condition.syntax() == expression.syntax())
}

/// Returns the condition owned by a direct `@if` or `@while` parent.
///
/// Example: `@while $i > 0 {}` owns `$i > 0`.
fn scss_control_condition(expression: &ScssExpression) -> Option<ScssExpression> {
    expression
        .parent::<ScssIfAtRule>()
        .and_then(|rule| rule.condition().ok())
        .or_else(|| {
            expression
                .parent::<ScssWhileAtRule>()
                .and_then(|rule| rule.condition().ok())
        })
}

/// Checks `$a == 0` inside transparent condition parens: `@if ($a == 0) {}`.
fn is_parenthesized_control_condition_expression(expression: &ScssExpression) -> bool {
    let Some(parenthesized) = expression.parent::<ScssParenthesizedExpression>() else {
        return false;
    };

    let Some(parent_expression) = parenthesized.parent::<ScssExpression>() else {
        return false;
    };

    is_direct_control_condition(&parent_expression)
        || is_parenthesized_control_condition_expression(&parent_expression)
}

#[cfg(test)]
mod tests {
    use biome_css_factory::{
        make,
        syntax::{
            AnyScssExpression, AnyScssExpressionItem, CssSyntaxKind, CssSyntaxNode, CssSyntaxToken,
            ScssParenthesizedExpression, T as CssT,
        },
    };
    use biome_rowan::AstNode;

    fn token(kind: CssSyntaxKind, text: &str) -> CssSyntaxToken {
        CssSyntaxToken::new_detached(kind, text, [], [])
    }

    fn value(text: &str) -> AnyScssExpression {
        AnyScssExpression::AnyCssValue(make::css_identifier(token(CssT![ident], text)).into())
    }

    fn item(text: &str) -> AnyScssExpressionItem {
        AnyScssExpressionItem::AnyCssValue(make::css_identifier(token(CssT![ident], text)).into())
    }

    fn sequence(items: Vec<AnyScssExpressionItem>) -> AnyScssExpression {
        make::scss_expression(make::scss_expression_item_list(items)).into()
    }

    fn parenthesized(value: AnyScssExpression) -> ScssParenthesizedExpression {
        make::scss_parenthesized_expression(token(CssT!['('], "("), value, token(CssT![')'], ")"))
    }

    #[test]
    fn omit_parentheses_traverses_direct_and_wrapped_layers() {
        let inner = sequence(vec![item("a")]);
        let direct: AnyScssExpression = parenthesized(parenthesized(inner.clone()).into()).into();
        let unwrapped = direct.omit_parentheses();
        assert_eq!(unwrapped.syntax().kind(), CssSyntaxKind::SCSS_EXPRESSION);
        assert_eq!(unwrapped.syntax().to_string(), "a");

        let nested = sequence(vec![parenthesized(inner).into()]);
        let wrapped = sequence(vec![parenthesized(nested).into()]);
        let unwrapped = wrapped.clone().omit_parentheses();
        assert_eq!(unwrapped.syntax().kind(), CssSyntaxKind::SCSS_EXPRESSION);
        assert_eq!(unwrapped.syntax().to_string(), "a");
        assert_eq!(wrapped.syntax().to_string(), "((a))");
    }

    #[test]
    fn omit_parentheses_preserves_non_parenthesized_payloads() {
        let list = make::scss_list_expression(make::scss_list_expression_element_list(
            [make::scss_list_expression_element(value("a"))],
            [token(CssT![,], ",")],
        ));
        let map = make::scss_map_expression(
            token(CssT!['('], "("),
            make::scss_map_expression_pair_list(
                [make::scss_map_expression_pair(
                    value("a"),
                    token(CssT![:], ":"),
                    value("b"),
                )],
                [],
            ),
            token(CssT![')'], ")"),
        );
        for inner in [
            value("a"),
            sequence(vec![]),
            sequence(vec![item("a")]),
            sequence(vec![parenthesized(value("a")).into(), item("b")]),
            list.into(),
            map.clone().into(),
            sequence(vec![map.into()]),
        ] {
            assert_eq!(inner.clone().omit_parentheses(), inner);
            let grouped = parenthesized(inner);
            let expected = grouped.expression().unwrap();
            assert_eq!(
                AnyScssExpression::from(grouped).omit_parentheses(),
                expected
            );
        }
    }

    #[test]
    fn omit_parentheses_stops_at_missing_contents() {
        let missing = ScssParenthesizedExpression::unwrap_cast(CssSyntaxNode::new_detached(
            CssSyntaxKind::SCSS_PARENTHESIZED_EXPRESSION,
            [
                Some(token(CssT!['('], "(").into()),
                None,
                Some(token(CssT![')'], ")").into()),
            ],
        ));
        let wrapped = sequence(vec![missing.clone().into()]);
        assert_eq!(wrapped.clone().omit_parentheses(), wrapped);

        let outer = parenthesized(missing.into());
        let expected = outer.expression().unwrap();
        assert_eq!(AnyScssExpression::from(outer).omit_parentheses(), expected);
    }

    #[test]
    fn omit_parentheses_uses_contents_without_a_closing_token() {
        let inner = sequence(vec![item("a")]);
        let missing_close = ScssParenthesizedExpression::unwrap_cast(CssSyntaxNode::new_detached(
            CssSyntaxKind::SCSS_PARENTHESIZED_EXPRESSION,
            [
                Some(token(CssT!['('], "(").into()),
                Some(inner.into_syntax().into()),
                None,
            ],
        ));
        let expected = missing_close.expression().unwrap();
        let expression: AnyScssExpression = missing_close.into();
        assert_eq!(expression.omit_parentheses(), expected);
    }
}
