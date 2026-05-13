use crate::{
    html::lists::attribute_list::FormatHtmlAttributeListOptions,
    prelude::*,
    utils::metadata::should_lowercase_html_tag,
    verbatim::{format_html_leading_comments, format_html_trailing_comments},
};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlSelfClosingElement, HtmlSelfClosingElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSelfClosingElement {
    /// Whether comments adjacent to this element are formatted by the containing child list.
    comments_as_children: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSelfClosingElementOptions {
    /// Whether comments adjacent to this element are formatted by the containing child list.
    pub comments_as_children: bool,
}

impl FormatRuleWithOptions<HtmlSelfClosingElement> for FormatHtmlSelfClosingElement {
    type Options = FormatHtmlSelfClosingElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.comments_as_children = options.comments_as_children;
        self
    }
}

impl FormatNodeRule<HtmlSelfClosingElement> for FormatHtmlSelfClosingElement {
    fn fmt_fields(&self, node: &HtmlSelfClosingElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlSelfClosingElementFields {
            l_angle_token,
            name,
            attributes,
            slash_token,
            r_angle_token,
        } = node.as_fields();
        let bracket_same_line = f.options().bracket_same_line().value();
        let self_close_void_elements = f.options().self_close_void_elements();
        let name = name?;
        let is_canonical_html_element = name
            .as_html_tag_name()
            .is_some_and(|name| should_lowercase_html_tag(f, name));

        write!(f, [l_angle_token.format(), name.format()])?;

        let attr_group_id = f.group_id("element-attr-group-id");
        write!(
            f,
            [&group(&format_with(|f| {
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
                if bracket_same_line {
                    let is_void = node.is_void_element().unwrap_or_default();
                    let will_have_slash = self_close_void_elements.is_always()
                        || (slash_token.is_some()
                            && !(is_void && self_close_void_elements.is_never()));

                    if will_have_slash {
                        write!(f, [hard_space()])?;
                    }
                } else if attributes.len() >= 1 {
                    if self_close_void_elements.is_always() {
                        write!(f, [soft_line_break_or_space()])?;
                    } else {
                        write!(f, [soft_line_break()])?;
                    }
                } else if self_close_void_elements.is_always() {
                    write!(f, [soft_line_break_or_space()])?;
                }

                if self_close_void_elements.is_always() {
                    // TODO: These tokens (the `/>`) are not yet borrowed by sibling elements for whitespace sensitivity.
                    // To resolve this, these tokens either need to be passed to or deferred to sibling text elements when
                    // whitespace sensitivity would require it.
                    if slash_token.is_some() {
                        write!(f, [if_group_fits_on_line(&space()), slash_token.format()])?;
                    } else {
                        write!(f, [if_group_fits_on_line(&space()), token("/")])?;
                    }
                }
                // We remove the slash only from void elements
                else if node.is_void_element().unwrap_or_default()
                    && self_close_void_elements.is_never()
                {
                    if let Some(slash_token) = &slash_token {
                        write!(f, [format_removed(slash_token)])?;
                    }
                } else {
                    if slash_token.is_some() {
                        // only add a space before the slash if it exists.
                        write!(f, [space()])?;
                    }
                    write!(f, [slash_token.format()])?;
                }

                write!(f, [r_angle_token.format()])?;
                Ok(())
            }))
            .with_group_id(Some(attr_group_id))]
        )?;

        Ok(())
    }

    fn fmt_leading_comments(
        &self,
        _node: &HtmlSelfClosingElement,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        if self.comments_as_children {
            return Ok(());
        }

        format_html_leading_comments(_node.syntax()).fmt(_f)
    }

    fn fmt_trailing_comments(
        &self,
        _node: &HtmlSelfClosingElement,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        if self.comments_as_children {
            return Ok(());
        }

        format_html_trailing_comments(_node.syntax()).fmt(_f)
    }
}
