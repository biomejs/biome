use crate::prelude::*;
use biome_formatter::{write, AttributePosition};
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

        write!(f, [l_angle_token.format(), name.format(), space()])?;
        let line_break = if f.options().attribute_position() == AttributePosition::Multiline {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        };

        let attr_group_id = f.group_id("element-attr-group-id");
        write!(
            f,
            [&group(&format_with(|f| {
                if attributes.len() > 0 {
                    write!(
                        f,
                        [
                            space(),
                            &soft_line_indent_or_space(&format_with(|f| {
                                f.join_with(&line_break)
                                    .entries(attributes.iter().formatted())
                                    .finish()?;

                                Ok(())
                            }))
                        ]
                    )?;
                }
                // Whitespace sensitivity takes precedence over bracketSameLine for correctness.
                //
                // The r_angle is placed inside this group because prettier always includes this token
                // in the same group as the attributes, unless the token is being borrowed.
                // When these tokens are borrowed, they are managed by the sibling `HtmlElementList` formatter.
                if bracket_same_line {
                    write!(f, [hard_space()])?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }

                // TODO: These tokens (the `/>`) are not yet borrowed by sibling elements for whitespace sensitivity.
                if slash_token.is_some() {
                    write!(f, [slash_token.format()])?;
                } else {
                    write!(f, [text("/")])?;
                }

                write!(f, [r_angle_token.format()])?;
                Ok(())
            }))
            .with_group_id(Some(attr_group_id))]
        )?;

        Ok(())
    }
}
