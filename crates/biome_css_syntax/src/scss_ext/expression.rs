use crate::{
    AnyCssExpression, AnyScssExpression, AnyScssExpressionItem, CssSyntaxNode, ScssExpression,
    ScssKeywordArgument,
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
