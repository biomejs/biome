use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlDoubleTextExpression, HtmlDoubleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDoubleTextExpression;
impl FormatNodeRule<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlDoubleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlDoubleTextExpressionFields {
            l_double_curly_token,
            expression,
            r_double_curly_token,
        } = node.as_fields();

        if is_multiline_double_text_expression(node) {
            return write!(
                f,
                [
                    l_double_curly_token.format(),
                    block_indent(&expression.format()),
                    r_double_curly_token.format(),
                ]
            );
        }

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

pub(crate) fn is_multiline_double_text_expression(node: &HtmlDoubleTextExpression) -> bool {
    node.expression()
        .ok()
        .and_then(|expression| expression.html_literal_token().ok())
        .is_some_and(|token| has_boundary_newline(token.text()))
}

fn has_boundary_newline(text: &str) -> bool {
    text.starts_with(['\n', '\r']) || text.ends_with(['\n', '\r'])
}
