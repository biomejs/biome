use crate::prelude::*;
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

        write!(
            f,
            [
                l_angle_token.format(),
                name.format(),
                attributes.format(),
                soft_line_break_or_space(),
                slash_token.format(),
                r_angle_token.format()
            ]
        )
    }
}
