use biome_css_syntax::{
    AnyCssExpression, AnyScssExpression, AnyScssExpressionItem, CssSyntaxNode, ScssExpression,
    ScssIncludeArgumentList, ScssKeywordArgument,
};
use biome_rowan::{AstNode, AstNodeList};

/// Returns `$arg: value` from a CSS expression that wraps an SCSS expression.
pub(crate) fn scss_keyword_argument_from_css_expression(
    value: &AnyCssExpression,
) -> Option<ScssKeywordArgument> {
    value
        .as_scss_expression()
        .cloned()
        .map(AnyScssExpression::ScssExpression)
        .and_then(|expression| scss_keyword_argument_from_expression(&expression))
}

/// Returns `$arg: value` when a syntax node is an SCSS expression wrapper.
pub(crate) fn scss_keyword_argument_from_syntax(
    node: &CssSyntaxNode,
) -> Option<ScssKeywordArgument> {
    AnyScssExpression::cast(node.clone())
        .and_then(|expression| scss_keyword_argument_from_expression(&expression))
}

/// Returns `$arg: value` when an SCSS expression only wraps that item.
pub(crate) fn scss_keyword_argument_from_expression(
    value: &AnyScssExpression,
) -> Option<ScssKeywordArgument> {
    unwrap_single_expression_item(value).and_then(|item| item.as_scss_keyword_argument().cloned())
}

/// Returns `true` for values that already decide their own internal layout,
/// such as `$arg: (a, b)` or `$arg: (key: value)`.
pub(crate) fn is_self_breaking_value(value: &AnyScssExpression) -> bool {
    matches!(
        value,
        AnyScssExpression::ScssListExpression(_)
            | AnyScssExpression::ScssMapExpression(_)
            | AnyScssExpression::ScssParenthesizedExpression(_)
    ) || unwrap_single_expression_item(value).is_some_and(|item| {
        matches!(
            item,
            AnyScssExpressionItem::ScssListExpression(_)
                | AnyScssExpressionItem::ScssMapExpression(_)
                | AnyScssExpressionItem::ScssParenthesizedExpression(_)
        )
    })
}

/// Finds a keyword argument before the surrounding include argument list.
///
/// Example: in `@include mix($arg: (a, b))`, `(a, b)` maps to `$arg`.
pub(crate) fn include_keyword_argument_before_argument_list(
    node: &CssSyntaxNode,
) -> Option<ScssKeywordArgument> {
    node.ancestors()
        .take_while(|ancestor| !ScssIncludeArgumentList::can_cast(ancestor.kind()))
        .find_map(ScssKeywordArgument::cast)
}

/// Unwraps `ScssExpression` containers that only wrap a single item.
pub(crate) fn unwrap_single_expression_item(
    value: &AnyScssExpression,
) -> Option<AnyScssExpressionItem> {
    value.as_scss_expression().and_then(single_expression_item)
}

/// Returns the only item in an SCSS expression when no separators or
/// operators widen it into a larger expression.
pub(crate) fn single_expression_item(expression: &ScssExpression) -> Option<AnyScssExpressionItem> {
    let mut items = expression.items().iter();
    let first = items.next()?;

    items.next().is_none().then_some(first)
}
