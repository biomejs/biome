use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AstroExpression, AstroExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroExpression;
impl FormatNodeRule<AstroExpression> for FormatAstroExpression {
    /// Formats the fields of an `AstroExpression` node by writing its left curly brace, expression, and right curly brace tokens in order.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `node` is an AstroExpression and `formatter` is a mutable HtmlFormatter:
    /// FormatAstroExpression::default().fmt_fields(&node, &mut formatter)?;
    /// ```
    fn fmt_fields(&self, node: &AstroExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AstroExpressionFields {
            l_curly_token,
            expression_token,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                expression_token.format(),
                r_curly_token.format()
            ]
        )
    }
}
