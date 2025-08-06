//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlAttributeInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlAttributeInitializer;
impl FormatRule<AnyHtmlAttributeInitializer> for FormatAnyHtmlAttributeInitializer {
    type Context = HtmlFormatContext;
    /// Formats an HTML attribute initializer node using the appropriate formatting rule.
    ///
    /// Delegates formatting based on the variant of the attribute initializer, supporting both single text expressions and string values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use biome_html_formatter::html::any::{FormatAnyHtmlAttributeInitializer, AnyHtmlAttributeInitializer};
    /// # use biome_html_formatter::HtmlFormatter;
    /// let rule = FormatAnyHtmlAttributeInitializer::default();
    /// let node = AnyHtmlAttributeInitializer::HtmlString(/* ... */);
    /// let mut formatter = HtmlFormatter::default();
    /// rule.fmt(&node, &mut formatter).unwrap();
    /// ```
    fn fmt(&self, node: &AnyHtmlAttributeInitializer, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlAttributeInitializer::HtmlSingleTextExpression(node) => node.format().fmt(f),
            AnyHtmlAttributeInitializer::HtmlString(node) => node.format().fmt(f),
        }
    }
}
