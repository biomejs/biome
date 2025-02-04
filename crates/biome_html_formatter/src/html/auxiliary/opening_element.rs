use crate::prelude::*;
use biome_formatter::{write, FormatRuleWithOptions};
use biome_html_syntax::{HtmlOpeningElement, HtmlOpeningElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlOpeningElement {
    /// Whether or not the r_angle is borrowed by the children of the element (aka [`HtmlElementList`][HtmlElementList]). See also: [`FormatHtmlElementList`][FormatHtmlElementList]
    ///
    /// In this context "borrowed" tokens refers to tokens that would normally be formatted by this formatter, but are instead formatted by the sibling `HtmlElementList`. In other words, borrowed tokens are managed by a different formatter, and must not be printed.
    /// This is necessary to get the correct tokens in the right groups so that we don't accidentally add whitespace inside elements when we shouldn't. See also: [`crate::context::WhitespaceSensitivity`].
    ///
    /// [FormatHtmlElementList]: crate::html::lists::element_list::FormatHtmlElementList
    /// [HtmlElementList]: biome_html_syntax::HtmlElementList
    r_angle_is_borrowed: bool,
}

pub(crate) struct FormatHtmlOpeningElementOptions {
    /// Whether or not the r_angle is borrowed, and therefore managed by a different formatter.
    pub r_angle_is_borrowed: bool,
}

impl FormatRuleWithOptions<HtmlOpeningElement> for FormatHtmlOpeningElement {
    type Options = FormatHtmlOpeningElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.r_angle_is_borrowed = options.r_angle_is_borrowed;
        self
    }
}

impl FormatNodeRule<HtmlOpeningElement> for FormatHtmlOpeningElement {
    fn fmt_fields(&self, node: &HtmlOpeningElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlOpeningElementFields {
            l_angle_token,
            name,
            attributes,
            r_angle_token,
        } = node.as_fields();

        let bracket_same_line = f.options().bracket_same_line().value();
        write!(f, [l_angle_token.format(), name.format()])?;

        let attr_group_id = f.group_id("element-attr-group-id");
        write!(
            f,
            [&group(&format_with(|f| {
                attributes.format().fmt(f)?;

                // Whitespace sensitivity takes precedence over bracketSameLine for correctness.
                //
                // The r_angle is placed inside this group because prettier always includes this token
                // in the same group as the attributes, unless the token is being borrowed.
                // When these tokens are borrowed, they are managed by the sibling `HtmlElementList` formatter.
                if !bracket_same_line {
                    write!(f, [soft_line_break()])?;
                }
                if !self.r_angle_is_borrowed {
                    write!(f, [r_angle_token.format()])?;
                }
                Ok(())
            }))
            .with_group_id(Some(attr_group_id))]
        )?;

        Ok(())
    }
}
