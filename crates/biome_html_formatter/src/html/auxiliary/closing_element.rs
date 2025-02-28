use crate::{prelude::*, utils::metadata::is_element_whitespace_sensitive};
use biome_formatter::{write, FormatRuleWithOptions};
use biome_html_syntax::{HtmlClosingElement, HtmlClosingElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlClosingElement {
    /// Whether or not the `</tag` part is borrowed by the children of the element (aka [`HtmlElementList`][HtmlElementList]). See also: [`FormatHtmlElementList`][FormatHtmlElementList]
    ///
    /// In this context "borrowed" tokens refers to tokens that would normally be formatted by this formatter, but are instead formatted by the sibling `HtmlElementList`.
    /// This is necessary to get the correct tokens in the right groups so that we don't accidentally add whitespace inside elements when we shouldn't. See also: [`crate::context::WhitespaceSensitivity]
    ///
    /// [FormatHtmlElementList]: crate::html::lists::element_list::FormatHtmlElementList
    /// [HtmlElementList]: biome_html_syntax::HtmlElementList
    tag_borrowed: bool,
}
pub(crate) struct FormatHtmlClosingElementOptions {
    /// Whether or not the `</tag` part of this tag is borrowed, and therefore managed by a different formatter.
    pub tag_borrowed: bool,
}

impl FormatRuleWithOptions<HtmlClosingElement> for FormatHtmlClosingElement {
    type Options = FormatHtmlClosingElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.tag_borrowed = options.tag_borrowed;
        self
    }
}

impl FormatNodeRule<HtmlClosingElement> for FormatHtmlClosingElement {
    fn fmt_fields(&self, node: &HtmlClosingElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlClosingElementFields {
            l_angle_token,
            name,
            slash_token,
            r_angle_token,
        } = node.as_fields();

        let name = name?;
        let is_whitespace_sensitive = is_element_whitespace_sensitive(f, &name);

        // When these tokens are borrowed, they are managed by the sibling `HtmlElementList` formatter.
        if !self.tag_borrowed {
            // Handle whitespace sensitivity in cases where the HtmlElementList formatter is not invoked because the element has no children.
            if let Ok(l_angle_token) = &l_angle_token {
                if is_whitespace_sensitive && l_angle_token.has_leading_whitespace_or_newline() {
                    // we can't get rid of the whitespace if the element is whitespace sensitive
                    write!(f, [space()])?;
                }
            }

            write!(
                f,
                [l_angle_token.format(), slash_token.format(), name.format()]
            )?;
        }

        write!(f, [r_angle_token.format()])?;

        Ok(())
    }
}
