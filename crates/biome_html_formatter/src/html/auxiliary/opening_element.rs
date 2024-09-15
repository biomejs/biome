use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlOpeningElement, HtmlOpeningElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlOpeningElement;
impl FormatNodeRule<HtmlOpeningElement> for FormatHtmlOpeningElement {
    fn fmt_fields(&self, node: &HtmlOpeningElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlOpeningElementFields {
            l_angle_token,
            name,
            attributes,
            r_angle_token,
        } = node.as_fields();

        write!(f, [l_angle_token.format(), name.format(),])?;
        if attributes.len() > 0 {
            write!(f, [space(), attributes.format()])?
        }
        write!(f, [r_angle_token.format()])?;

        Ok(())
    }
}
