use crate::prelude::*;
use biome_css_syntax::{CssParenthesizedExpression, CssParenthesizedExpressionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParenthesizedExpression;
impl FormatNodeRule<CssParenthesizedExpression> for FormatCssParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &CssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&expression.format()),
                r_paren_token.format()
            ])]
        )
    }
}
