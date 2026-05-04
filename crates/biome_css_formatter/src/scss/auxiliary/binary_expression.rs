use crate::prelude::*;
use biome_css_syntax::{
    ScssBinaryExpression, ScssBinaryExpressionFields, ScssParenthesizedExpression,
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
        let should_indent_right = is_nested_in_parenthesized_expression(node);
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

fn is_nested_in_parenthesized_expression(node: &ScssBinaryExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| ScssParenthesizedExpression::can_cast(ancestor.kind()))
}
