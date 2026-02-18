use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{
    HtmlAttributeDoubleTextExpression, HtmlAttributeDoubleTextExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeDoubleTextExpression;

impl FormatNodeRule<HtmlAttributeDoubleTextExpression> for FormatHtmlAttributeDoubleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlAttributeDoubleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAttributeDoubleTextExpressionFields {
            l_double_curly_token,
            expression,
            r_double_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_double_curly_token.format(),
                space(),
                expression.format(),
                space(),
                r_double_curly_token.format(),
            ]
        )
    }
}
