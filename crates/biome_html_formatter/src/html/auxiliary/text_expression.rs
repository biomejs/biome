use crate::prelude::*;
use biome_html_syntax::HtmlTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression;
impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    /// Formats an `HtmlTextExpression` node using verbatim HTML formatting.
    ///
    /// Delegates formatting of the entire syntax node to `format_html_verbatim_node`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::{HtmlTextExpression, HtmlFormatter, FormatHtmlTextExpression};
    /// # let node = HtmlTextExpression::parse("{{ expression }}").unwrap();
    /// # let mut formatter = HtmlFormatter::new();
    /// let rule = FormatHtmlTextExpression::default();
    /// rule.fmt_fields(&node, &mut formatter).unwrap();
    /// ```
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
