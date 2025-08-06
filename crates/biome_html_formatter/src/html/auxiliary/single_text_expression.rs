use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlSingleTextExpression, HtmlSingleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSingleTextExpression;
impl FormatNodeRule<HtmlSingleTextExpression> for FormatHtmlSingleTextExpression {
    /// Formats the fields of an `HtmlSingleTextExpression` node by writing the left curly token, inner expression, and right curly token in order.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `node` is an HtmlSingleTextExpression and `formatter` is a mutable HtmlFormatter:
    /// let rule = FormatHtmlSingleTextExpression::default();
    /// rule.fmt_fields(&node, &mut formatter)?;
    /// ```
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
