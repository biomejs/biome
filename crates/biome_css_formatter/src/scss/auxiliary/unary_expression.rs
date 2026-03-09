use crate::prelude::*;
use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::{ScssUnaryExpression, ScssUnaryExpressionFields, T};
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

        if needs_space {
            write!(f, [operator.format(), space(), expression.format()])
        } else {
            write!(f, [operator.format(), expression.format()])
        }
    }
}
