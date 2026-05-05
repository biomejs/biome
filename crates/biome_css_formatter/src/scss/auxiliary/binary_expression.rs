use crate::prelude::*;
use biome_css_syntax::{
    ScssBinaryExpression, ScssBinaryExpressionFields, ScssParenthesizedExpression,
    is_in_scss_control_condition_sequence,
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
        let formatted_right = format_with(|f| {
            if is_nested_parenthesized {
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

/// Detects nested binary values inside `(...)`, such as `($a + $b)`.
fn is_nested_in_parenthesized_expression(node: &ScssBinaryExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| ScssParenthesizedExpression::can_cast(ancestor.kind()))
}
