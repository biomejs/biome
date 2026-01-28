use crate::{
    html::lists::attribute_list::FormatHtmlAttributeListOptions,
    prelude::*,
    utils::{css_display::get_css_display_from_tag, metadata::should_lowercase_html_tag},
};
use biome_formatter::{FormatRuleWithOptions, GroupId, write};
use biome_html_syntax::{HtmlOpeningElement, HtmlOpeningElementFields, HtmlSyntaxToken};
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

    attr_group_id: Option<GroupId>,

    /// A `>` token borrowed from the previous sibling element's closing tag.
    /// When two inline elements are adjacent with no whitespace (`</span><span`),
    /// Prettier borrows the `>` from the first element's closing tag and prints it
    /// at the start of the next element's opening tag group to keep them "touching".
    borrowed_sibling_r_angle: Option<HtmlSyntaxToken>,
}

pub(crate) struct FormatHtmlOpeningElementOptions {
    /// Whether or not the r_angle is borrowed, and therefore managed by a different formatter.
    pub r_angle_is_borrowed: bool,

    pub attr_group_id: GroupId,

    /// A `>` token borrowed from the previous sibling element's closing tag.
    pub borrowed_sibling_r_angle: Option<HtmlSyntaxToken>,
}

impl FormatRuleWithOptions<HtmlOpeningElement> for FormatHtmlOpeningElement {
    type Options = FormatHtmlOpeningElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.r_angle_is_borrowed = options.r_angle_is_borrowed;
        self.attr_group_id = Some(options.attr_group_id);
        self.borrowed_sibling_r_angle = options.borrowed_sibling_r_angle;
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

        let l_angle_token = l_angle_token?;
        let name = name?;
        let css_display = get_css_display_from_tag(&name);
        let is_whitespace_sensitive = css_display.is_internally_whitespace_sensitive(f);
        let is_canonical_html_element = name
            .as_html_tag_name()
            .is_some_and(|name| should_lowercase_html_tag(f, name));

        let bracket_same_line = f.options().bracket_same_line().value();

        // Capture the borrowed sibling r_angle for use in the closure
        let borrowed_sibling_r_angle = self.borrowed_sibling_r_angle.clone();

        write!(
            f,
            [&group(&format_with(|f| {
                // Print borrowed `>` from previous sibling's closing tag at the start of this group.
                // This implements Prettier's pattern: group([">", "<span", ...attrs])
                // where the `>` is borrowed from `</span>` of the previous sibling element.
                if let Some(ref borrowed_r_angle) = borrowed_sibling_r_angle {
                    write!(f, [borrowed_r_angle.format()])?;
                }
                write!(f, [l_angle_token.format(), name.format()])?;
                attributes
                    .format()
                    .with_options(FormatHtmlAttributeListOptions {
                        is_canonical_html_element,
                        tag_name: Some(name.clone()),
                    })
                    .fmt(f)?;

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
            .with_group_id(self.attr_group_id)]
        )?;

        // Handle whitespace sensitivity in cases where the HtmlElementList formatter is not invoked because the element has no children.
        if let Ok(r_angle_token) = &r_angle_token
            && is_whitespace_sensitive
            && r_angle_token.has_trailing_whitespace()
        {
            // we can't get rid of the whitespace if the element is whitespace sensitive
            write!(f, [space()])?;
        }

        Ok(())
    }
}
