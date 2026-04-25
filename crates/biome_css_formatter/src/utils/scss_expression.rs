use biome_css_syntax::{AnyScssExpression, AnyScssExpressionItem, ScssExpression};
use biome_rowan::AstNodeList;

/// Unwraps `ScssExpression` containers that only wrap a single item.
pub(crate) fn unwrap_single_expression_item(
    value: &AnyScssExpression,
) -> Option<AnyScssExpressionItem> {
    value.as_scss_expression().and_then(single_expression_item)
}

/// Returns the only item in an SCSS expression when no separators or
/// operators widen it into a larger expression.
fn single_expression_item(expression: &ScssExpression) -> Option<AnyScssExpressionItem> {
    let mut items = expression.items().iter();
    let first = items.next()?;

    items.next().is_none().then_some(first.clone())
}
