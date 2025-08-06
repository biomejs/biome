//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlTextExpression;
impl FormatRule<AnyHtmlTextExpression> for FormatAnyHtmlTextExpression {
    type Context = HtmlFormatContext;
    /// Formats an `AnyHtmlTextExpression` node according to its specific variant.
    ///
    /// Delegates formatting to the corresponding variant's formatter. Returns any formatting errors encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_html_syntax::AnyHtmlTextExpression;
    /// use biome_html_formatter::{FormatRule, HtmlFormatContext};
    ///
    /// let node: AnyHtmlTextExpression = /* obtain node */;
    /// let mut formatter = HtmlFormatContext::default().create_formatter();
    /// FormatAnyHtmlTextExpression.fmt(&node, &mut formatter)?;
    /// ```
    fn fmt(&self, node: &AnyHtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlTextExpression::HtmlBogusTextExpression(node) => node.format().fmt(f),
            AnyHtmlTextExpression::HtmlDoubleTextExpression(node) => node.format().fmt(f),
            AnyHtmlTextExpression::HtmlSingleTextExpression(node) => node.format().fmt(f),
        }
    }
}
