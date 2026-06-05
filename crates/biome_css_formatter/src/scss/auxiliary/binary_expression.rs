use crate::prelude::*;
use biome_css_syntax::{
    ScssBinaryExpression, ScssBinaryExpressionFields, is_in_scss_control_condition_sequence,
    is_in_scss_parenthesized_expression, is_scss_parenthesized_expression,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let left = node.left();
        let formatted_right = FormatScssBinaryRightSide::new(node);

        if is_in_scss_control_condition_sequence(node) {
            write!(f, [left.format(), formatted_right])
        } else {
            write!(f, [group(&format_args![left.format(), formatted_right])])
        }
    }
}

/// Formats the operator and right side of a SCSS binary expression.
///
/// Biome normalizes source-tight operators to `left <op> right`, even when
/// Prettier preserves them because of parser token shapes.
struct FormatScssBinaryRightSide<'a> {
    node: &'a ScssBinaryExpression,
}

impl<'a> FormatScssBinaryRightSide<'a> {
    fn new(node: &'a ScssBinaryExpression) -> Self {
        Self { node }
    }

    fn should_indent(&self) -> bool {
        let ScssBinaryExpressionFields { right, .. } = self.node.as_fields();

        is_in_scss_parenthesized_expression(self.node)
            || right.as_ref().is_ok_and(is_scss_parenthesized_expression)
    }
}

impl Format<CssFormatContext> for FormatScssBinaryRightSide<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            operator, right, ..
        } = self.node.as_fields();
        let should_indent = self.should_indent();

        if should_indent {
            write!(
                f,
                [
                    space(),
                    operator.format(),
                    indent(&format_args![soft_line_break_or_space(), right.format()])
                ]
            )
        } else {
            write!(
                f,
                [
                    space(),
                    operator.format(),
                    soft_line_break_or_space(),
                    right.format()
                ]
            )
        }
    }
}
