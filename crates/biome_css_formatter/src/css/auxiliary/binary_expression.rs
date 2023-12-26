use crate::prelude::*;
use biome_css_syntax::{CssBinaryExpression, CssBinaryExpressionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBinaryExpression;
impl FormatNodeRule<CssBinaryExpression> for FormatCssBinaryExpression {
    fn fmt_fields(&self, node: &CssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBinaryExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                operator_token.format(),
                soft_line_break_or_space(),
                right.format()
            ]
        )
    }
}
