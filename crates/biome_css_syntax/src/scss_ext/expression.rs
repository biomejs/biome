use crate::CssSyntaxKind::{SCSS_IF_AT_RULE, SCSS_WHILE_AT_RULE};
use crate::{
    AnyCssExpression, AnyScssExpression, AnyScssExpressionItem, CssLanguage, CssSyntaxNode,
    ScssExpression, ScssIfAtRule, ScssKeywordArgument, ScssParenthesizedExpression,
    ScssWhileAtRule,
};
use biome_rowan::{AstNode, AstNodeList};

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
    expression.syntax().parent().is_some_and(|parent| {
        let condition = match parent.kind() {
            SCSS_IF_AT_RULE => ScssIfAtRule::unwrap_cast(parent).condition(),
            SCSS_WHILE_AT_RULE => ScssWhileAtRule::unwrap_cast(parent).condition(),
            _ => return false,
        };

        condition.is_ok_and(|condition| condition.syntax() == expression.syntax())
    })
}

/// Checks `$a == 0` inside transparent condition parens: `@if ($a == 0) {}`.
fn is_parenthesized_control_condition_expression(expression: &ScssExpression) -> bool {
    let Some(parenthesized) = expression
        .syntax()
        .parent()
        .and_then(ScssParenthesizedExpression::cast)
    else {
        return false;
    };

    let Some(parent_expression) = parenthesized
        .syntax()
        .parent()
        .and_then(ScssExpression::cast)
    else {
        return false;
    };

    is_direct_control_condition(&parent_expression)
        || is_parenthesized_control_condition_expression(&parent_expression)
}
