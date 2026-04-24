use crate::prelude::*;
use biome_css_syntax::{CssUnaryExpression, CssUnaryExpressionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnaryExpression;
impl FormatNodeRule<CssUnaryExpression> for FormatCssUnaryExpression {
    fn fmt_fields(&self, node: &CssUnaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnaryExpressionFields {
            operator,
            expression,
        } = node.as_fields();
        let operator = operator?;
        let expression = expression?;

        write!(f, [operator.format(), expression.format()])
    }
}
