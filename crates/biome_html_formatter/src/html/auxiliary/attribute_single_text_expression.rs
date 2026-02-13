use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{
    HtmlAttributeSingleTextExpression, HtmlAttributeSingleTextExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeSingleTextExpression;

impl FormatNodeRule<HtmlAttributeSingleTextExpression> for FormatHtmlAttributeSingleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlAttributeSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAttributeSingleTextExpressionFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
