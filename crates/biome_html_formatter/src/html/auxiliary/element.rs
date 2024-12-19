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

        let tag_name = opening_element
            .clone()
            .and_then(|e| e.name())
            .map(|e| e.to_string())
            .unwrap_or_default();
        // `pre` tags are "preformatted", so we should not format the content inside them. https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre
        // We ignore the `script` and `style` tags as well, since embedded language parsing/formatting is not yet implemented.
        let should_be_verbatim = ["script", "style", "pre"]
            .iter()
            .any(|tag| tag_name.eq_ignore_ascii_case(tag));

        write!(f, [opening_element.format()])?;
        if should_be_verbatim {
            format_verbatim_skipped(children.syntax()).fmt(f)?;
            write!(f, [hard_line_break()])?;
        } else {
            write!(f, [children.format()])?;
        }
        write!(f, [closing_element.format()])?;

        Ok(())
    }
}
