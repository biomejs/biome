use crate::prelude::*;
use biome_css_syntax::{
    AnyCssValue, AnyScssExpression, CssSyntaxToken, ScssParenthesizedExpression,
    ScssUnaryExpression, ScssUnaryExpressionFields, T, is_in_scss_control_condition_sequence,
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
        if is_spaced_not_expression {
            let separator = format_with(|f| {
                if is_in_scss_control_condition_sequence(node) {
                    write!(f, [soft_line_break_or_space()])
                } else {
                    write!(f, [space()])
                }
            });

            write!(
                f,
                [
                    operator.format().with_text_case(CssCase::Preserve),
                    separator,
                    expression.format().with_text_case(CssCase::Preserve)
                ]
            )
        } else if needs_space_after_minus(&operator, &expression) {
            write!(
                f,
                [
                    operator.format().with_text_case(CssCase::Preserve),
                    space(),
                    expression.format().with_text_case(CssCase::Preserve)
                ]
            )
        } else {
            write!(
                f,
                [
                    operator.format().with_text_case(CssCase::Preserve),
                    expression.format().with_text_case(CssCase::Preserve)
                ]
            )
        }
    }
}

/// Keeps unary `-` separate from function names and module namespaces.
fn needs_space_after_minus(operator: &CssSyntaxToken, expression: &AnyScssExpression) -> bool {
    if operator.kind() != T![-] {
        return false;
    }

    match expression.as_any_css_value() {
        Some(AnyCssValue::ScssModuleMemberAccess(_)) => true,
        Some(AnyCssValue::ScssInterpolatedValue(value)) => value
            .items()
            .first()
            .is_some_and(|part| part.as_scss_namespaced_variable().is_some()),
        Some(AnyCssValue::AnyCssFunction(function)) => {
            operator.text_trimmed_range().end() < function.syntax().text_trimmed_range().start()
        }
        _ => false,
    }
}
