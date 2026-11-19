use crate::prelude::*;
use biome_css_syntax::{
    AnyCssValue, AnyScssExpression, CssSyntaxToken, ScssParenthesizedExpression,
    ScssSupportsDeclaration, ScssUnaryExpression, ScssUnaryExpressionFields, T,
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
        } else if needs_space_after_minus(node, &operator, &expression) {
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

/// Keeps unary `-` separate where joining it changes parsing.
fn needs_space_after_minus(
    node: &ScssUnaryExpression,
    operator: &CssSyntaxToken,
    expression: &AnyScssExpression,
) -> bool {
    if operator.kind() != T![-] {
        return false;
    }

    // Joining `- -space` into `--space` makes Sass preserve the supports value
    // as raw custom-property text instead of evaluating it.
    if expression
        .syntax()
        .first_token()
        .is_some_and(|token| token.text_trimmed().starts_with('-'))
        && is_in_supports_declaration_name(node)
    {
        return true;
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

/// Distinguishes a computed supports name from its declaration value.
///
/// ```scss
/// $gap: 2px;
/// @supports (- -space: $gap) {}
/// ```
fn is_in_supports_declaration_name(node: &ScssUnaryExpression) -> bool {
    let Some(declaration) = node
        .syntax()
        .ancestors()
        .find_map(ScssSupportsDeclaration::cast)
    else {
        return false;
    };

    declaration.name().is_ok_and(|name| {
        name.syntax()
            .text_trimmed_range()
            .contains_range(node.syntax().text_trimmed_range())
    })
}
