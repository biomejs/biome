use crate::prelude::*;
use biome_css_syntax::{ScssParenthesizedExpression, ScssParenthesizedExpressionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParenthesizedExpression;
impl FormatNodeRule<ScssParenthesizedExpression> for FormatScssParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &ScssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssParenthesizedExpressionFields {
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
