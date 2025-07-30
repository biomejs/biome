use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlTextExpression, HtmlTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression;
impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlTextExpressionFields {
            l_double_curly_token,
            expression_token,
            r_double_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_double_curly_token.format(),
                expression_token.format(),
                r_double_curly_token.format()
            ]
        )
    }
}
