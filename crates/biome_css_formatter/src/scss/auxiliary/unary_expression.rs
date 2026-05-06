use crate::prelude::*;
use biome_css_syntax::{
    AnyCssFunction, ScssParenthesizedExpression, ScssUnaryExpression, ScssUnaryExpressionFields, T,
    is_in_scss_control_condition_sequence,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUnaryExpression;
impl FormatNodeRule<ScssUnaryExpression> for FormatScssUnaryExpression {
    fn fmt_fields(&self, node: &ScssUnaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssUnaryExpressionFields {
            operator,
            expression,
        } = node.as_fields();
        let operator = operator?;
        let expression = expression?;

        let is_parenthesized = ScssParenthesizedExpression::can_cast(expression.syntax().kind());
        let is_spaced_not_expression = matches!(operator.kind(), T![not]) && !is_parenthesized;
        let is_source_spaced_minus_function = matches!(operator.kind(), T![-])
            && AnyCssFunction::can_cast(expression.syntax().kind())
            && operator.has_trailing_whitespace();

        if is_spaced_not_expression {
            let separator = format_with(|f| {
                if is_in_scss_control_condition_sequence(node) {
                    write!(f, [soft_line_break_or_space()])
                } else {
                    write!(f, [space()])
                }
            });

            write!(f, [operator.format(), separator, expression.format()])
        } else if is_source_spaced_minus_function {
            // Prettier keeps the source space in `- pow()`.
            write!(f, [operator.format(), space(), expression.format()])
        } else {
            write!(f, [operator.format(), expression.format()])
        }
    }
}
