use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlSingleTextExpression, HtmlSingleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSingleTextExpression;
impl FormatNodeRule<HtmlSingleTextExpression> for FormatHtmlSingleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlSingleTextExpressionFields {
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
