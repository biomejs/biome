use crate::prelude::*;
use biome_css_syntax::{
    AnyScssExpression, AnyScssExpressionItem, ScssBinaryExpression, ScssBinaryExpressionFields,
    ScssParenthesizedExpression, is_in_scss_control_condition_sequence,
    unwrap_single_expression_item,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();
        let is_nested_parenthesized = is_nested_in_parenthesized_expression(node);
        // `$a * ($b + $c)` lets the parentheses own the nested break.
        let should_indent_right =
            is_nested_parenthesized || right.as_ref().is_ok_and(is_parenthesized_expression);
        let formatted_right = format_with(|f| {
            if should_indent_right {
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break_or_space(),
                        right.format()
                    ])]
                )
            } else {
                write!(f, [soft_line_break_or_space(), right.format()])
            }
        });

        if is_in_scss_control_condition_sequence(node) {
            write!(
                f,
                [format_args![
                    left.format(),
                    space(),
                    operator.format(),
                    formatted_right
                ]]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    left.format(),
                    space(),
                    operator.format(),
                    formatted_right
                ])]
            )
        }
    }
}

/// Detects parenthesized operands, such as the right side in `$a * ($b)`.
fn is_parenthesized_expression(expression: &AnyScssExpression) -> bool {
    matches!(
        expression,
        AnyScssExpression::ScssParenthesizedExpression(_)
    ) || unwrap_single_expression_item(expression)
        .is_some_and(|item| matches!(item, AnyScssExpressionItem::ScssParenthesizedExpression(_)))
}

/// Detects nested binary values inside `(...)`, such as `($a + $b)`.
fn is_nested_in_parenthesized_expression(node: &ScssBinaryExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| ScssParenthesizedExpression::can_cast(ancestor.kind()))
}
