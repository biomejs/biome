use crate::prelude::*;
use biome_css_syntax::{
    AnyCssFunction, CssSyntaxKind, ScssIfAtRule, ScssUnaryExpression, ScssUnaryExpressionFields, T,
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

        let is_parenthesized = matches!(
            expression.syntax().kind(),
            CssSyntaxKind::SCSS_PARENTHESIZED_EXPRESSION
        );
        let needs_space = matches!(operator.kind(), T![not]) && !is_parenthesized;
        let keeps_minus_function_space = matches!(operator.kind(), T![-])
            && AnyCssFunction::can_cast(expression.syntax().kind())
            && operator.has_trailing_whitespace();

        if needs_space {
            if is_in_scss_if_condition(node) {
                write!(
                    f,
                    [
                        operator.format(),
                        soft_line_break_or_space(),
                        expression.format()
                    ]
                )
            } else {
                write!(f, [operator.format(), space(), expression.format()])
            }
        } else if keeps_minus_function_space {
            // Prettier keeps the source space in `- pow()`.
            write!(f, [operator.format(), space(), expression.format()])
        } else {
            write!(f, [operator.format(), expression.format()])
        }
    }
}

fn is_in_scss_if_condition(node: &ScssUnaryExpression) -> bool {
    let range = node.syntax().text_trimmed_range();

    node.syntax().ancestors().skip(1).any(|ancestor| {
        ScssIfAtRule::cast_ref(&ancestor)
            .and_then(|if_rule| if_rule.condition().ok())
            .is_some_and(|condition| {
                condition
                    .syntax()
                    .text_trimmed_range()
                    .contains_range(range)
            })
    })
}
