//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlAttribute;
impl FormatRule<AnyHtmlAttribute> for FormatAnyHtmlAttribute {
    type Context = HtmlFormatContext;
    /// Formats an `AnyHtmlAttribute` node using the appropriate formatter for its variant.
    ///
    /// Dispatches formatting based on whether the node is a standard HTML attribute, a bogus attribute, or a double text expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_html_syntax::AnyHtmlAttribute;
    /// use biome_html_formatter::{FormatAnyHtmlAttribute, HtmlFormatContext};
    ///
    /// let attribute: AnyHtmlAttribute = /* obtain or construct an attribute node */;
    /// let mut formatter = HtmlFormatContext::default().create_formatter();
    /// let rule = FormatAnyHtmlAttribute::default();
    /// rule.fmt(&attribute, &mut formatter).unwrap();
    /// ```
    fn fmt(&self, node: &AnyHtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlAttribute::HtmlAttribute(node) => node.format().fmt(f),
            AnyHtmlAttribute::HtmlBogusAttribute(node) => node.format().fmt(f),
            AnyHtmlAttribute::HtmlDoubleTextExpression(node) => node.format().fmt(f),
        }
    }
}
