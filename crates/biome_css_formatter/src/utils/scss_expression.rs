use biome_css_syntax::{AnyScssExpression, AnyScssExpressionItem, unwrap_single_expression_item};

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
