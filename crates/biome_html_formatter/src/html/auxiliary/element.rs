use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlElement, HtmlElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElement;
impl FormatNodeRule<HtmlElement> for FormatHtmlElement {
    fn fmt_fields(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        write!(
            f,
            [
                opening_element.format(),
                children.format(),
                closing_element.format(),
            ]
        )?;

        Ok(())
    }
}
