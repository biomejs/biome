use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlClosingElement, HtmlClosingElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlClosingElement;
impl FormatNodeRule<HtmlClosingElement> for FormatHtmlClosingElement {
    fn fmt_fields(&self, node: &HtmlClosingElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlClosingElementFields {
            l_angle_token,
            name,
            slash_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                name.format(),
                r_angle_token.format(),
            ]
        )?;

        Ok(())
    }
}
