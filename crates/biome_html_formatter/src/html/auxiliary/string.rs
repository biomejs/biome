use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::{HtmlString, HtmlStringFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlString;
impl FormatNodeRule<HtmlString> for FormatHtmlString {
    fn fmt_fields(&self, node: &HtmlString, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlStringFields { value_token } = node.as_fields();

        // Prettier always uses double quotes for HTML strings, regardless of configuration.
        if let Ok(value) = value_token.as_ref() {
            let value_text = value.text().trim();

            if !(value_text.starts_with('"') && value_text.ends_with('"')) {
                let range = if value_text.starts_with('\'') && value_text.ends_with('\'') {
                    value.text_range().add_start(1.into()).sub_end(1.into())
                } else {
                    value.text_range()
                };
                write!(
                    f,
                    [format_replaced(
                        value,
                        &group(&format_args![
                            text("\""),
                            located_token_text(value, range),
                            text("\""),
                        ])
                    )]
                )?;
                return Ok(());
            }
        }

        write!(f, [value_token.format()])
    }
}
