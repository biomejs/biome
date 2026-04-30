use crate::prelude::*;
use biome_css_syntax::{CssIfTestBooleanExprInParens, CssIfTestBooleanExprInParensFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfTestBooleanExprInParens;

impl FormatNodeRule<CssIfTestBooleanExprInParens> for FormatCssIfTestBooleanExprInParens {
    fn fmt_fields(
        &self,
        node: &CssIfTestBooleanExprInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssIfTestBooleanExprInParensFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(&expression.format(), should_insert_space),
                r_paren_token.format()
            ]
        )
    }
}
