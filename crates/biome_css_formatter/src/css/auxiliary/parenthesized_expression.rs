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

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(&expression.format(), should_insert_space),
                r_paren_token.format()
            ])]
        )
    }
}
