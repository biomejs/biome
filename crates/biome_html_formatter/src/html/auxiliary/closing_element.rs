use crate::{prelude::*, utils::css_display::get_css_display_from_tag};
use biome_formatter::{FormatRuleWithOptions, write};
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
    /// Whether or not the closing `>` is borrowed by an adjacent sibling element.
    /// When true, the `>` will be printed by the next sibling element instead of this closing tag.
    r_angle_borrowed: bool,
}
pub(crate) struct FormatHtmlClosingElementOptions {
    /// Whether or not the `</tag` part of this tag is borrowed, and therefore managed by a different formatter.
    pub tag_borrowed: bool,
    /// Whether or not the closing `>` is borrowed by an adjacent sibling element.
    pub r_angle_borrowed: bool,
}

impl FormatRuleWithOptions<HtmlClosingElement> for FormatHtmlClosingElement {
    type Options = FormatHtmlClosingElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.tag_borrowed = options.tag_borrowed;
        self.r_angle_borrowed = options.r_angle_borrowed;
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
        let css_display = get_css_display_from_tag(&name);
        let is_whitespace_sensitive = css_display.is_internally_whitespace_sensitive(f);

        // When these tokens are borrowed, they are managed by the sibling `HtmlElementList` formatter.
        if !self.tag_borrowed {
            // Handle whitespace sensitivity in cases where the HtmlElementList formatter is not invoked because the element has no children.
            if let Ok(l_angle_token) = &l_angle_token
                && is_whitespace_sensitive
                && l_angle_token.has_leading_whitespace_or_newline()
            {
                // we can't get rid of the whitespace if the element is whitespace sensitive
                write!(f, [space()])?;
            }

            write!(
                f,
                [l_angle_token.format(), slash_token.format(), name.format()]
            )?;
        }

        // When r_angle_borrowed is true, the `>` will be printed by the next sibling element
        if !self.r_angle_borrowed {
            write!(f, [r_angle_token.format()])?;
        }

        Ok(())
    }
}
