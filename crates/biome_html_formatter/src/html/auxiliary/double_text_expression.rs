use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlDoubleTextExpression, HtmlDoubleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDoubleTextExpression;
impl FormatNodeRule<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    /// Formats the fields of an `HtmlDoubleTextExpression` node by writing its left double curly token, contained expression, and right double curly token in order.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `node` is an HtmlDoubleTextExpression and `formatter` is an HtmlFormatter:
    /// let rule = FormatHtmlDoubleTextExpression::default();
    /// rule.fmt_fields(&node, &mut formatter)?;
    /// ```
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

        write!(
            f,
            [
                l_double_curly_token.format(),
                expression.format(),
                r_double_curly_token.format(),
            ]
        )
    }
}
