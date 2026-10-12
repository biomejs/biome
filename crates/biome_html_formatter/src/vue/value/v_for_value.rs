use crate::html::auxiliary::text_expression::FormatHtmlTextExpressionOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForValue, VueVForValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForValue;
impl FormatNodeRule<VueVForValue> for FormatVueVForValue {
    fn fmt_fields(&self, node: &VueVForValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForValueFields {
            l_quote,
            binding,
            operator,
            expression,
            r_quote,
        } = node.as_fields();
        let expression = expression?;
        let is_embedded = f.context().should_delegate_fmt_embedded_nodes()
            && f.context().is_embedded_node_range(expression.range());

        // The value is written between double quotes, unless its expression
        // isn't formatted as JavaScript and contains a double quote. That
        // expression is written as it is in the source, so the value keeps its
        // own quotes. The binding can't contain the quote of the value, which
        // would end the value.
        if !is_embedded && expression.syntax().text_trimmed().contains_char('"') {
            return write!(
                f,
                [
                    l_quote.format(),
                    binding.format(),
                    space(),
                    operator.format(),
                    space(),
                    expression.format(),
                    r_quote.format()
                ]
            );
        }

        // The JavaScript formatter writes strings with single quotes in
        // attribute values. When it can't format the expression, its source
        // text is written instead, with its double quotes escaped.
        write!(
            f,
            [
                format_replaced(&l_quote?, &token("\"")),
                binding.format(),
                space(),
                operator.format(),
                space(),
                expression
                    .format()
                    .with_options(FormatHtmlTextExpressionOptions {
                        escape_double_quotes: true,
                        ..Default::default()
                    }),
                format_replaced(&r_quote?, &token("\""))
            ]
        )
    }
}
