use crate::{
    html::lists::attribute_list::FormatHtmlAttributeListOptions, prelude::*,
    utils::metadata::is_canonical_html_tag,
};
use biome_formatter::write;
use biome_html_syntax::{HtmlSelfClosingElement, HtmlSelfClosingElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSelfClosingElement;
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
        let is_canonical_html_tag = is_canonical_html_tag(&name);

        write!(f, [l_angle_token.format(), name.format()])?;

        let attr_group_id = f.group_id("element-attr-group-id");
        write!(
            f,
            [&group(&format_with(|f| {
                attributes
                    .format()
                    .with_options(FormatHtmlAttributeListOptions {
                        is_canonical_html_element: is_canonical_html_tag,
                        tag_name: Some(name.clone()),
                    })
                    .fmt(f)?;

                // Whitespace sensitivity takes precedence over bracketSameLine for correctness.
                //
                // The r_angle is placed inside this group because prettier always includes this token
                // in the same group as the attributes, unless the token is being borrowed.
                // When these tokens are borrowed, they are managed by the sibling `HtmlElementList` formatter.
                if bracket_same_line {
                    write!(f, [hard_space()])?;
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
                        write!(f, [slash_token.format()])?;
                    } else {
                        write!(f, [text("/")])?;
                    }
                }
                // We remove the slash only from void elements
                else if node.is_void_element()? && self_close_void_elements.is_never() {
                    if let Some(slash_token) = &slash_token {
                        write!(f, [format_removed(slash_token)])?;
                    }
                } else {
                    write!(f, [slash_token.format()])?;
                }

                write!(f, [r_angle_token.format()])?;
                Ok(())
            }))
            .with_group_id(Some(attr_group_id))]
        )?;

        Ok(())
    }
}
