use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::{HtmlString, HtmlStringFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlString;
impl FormatNodeRule<HtmlString> for FormatHtmlString {
    fn fmt_fields(&self, node: &HtmlString, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlStringFields { value_token } = node.as_fields();

        // Prettier always uses double quotes for HTML strings, regardless of configuration.
        // Unless the string contains a double quote, in which case it uses single quotes.
        if let Ok(value) = value_token.as_ref() {
            let value_text = value.text_trimmed();

            if !(value_text.starts_with('"') && value_text.ends_with('"')) {
                let contains_double_quote = value_text.contains('"');

                let range = if value_text.starts_with('\'')
                    && value_text.ends_with('\'')
                    && !contains_double_quote
                {
                    value
                        .text_trimmed_range()
                        .add_start(1.into())
                        .sub_end(1.into())
                } else {
                    value.text_trimmed_range()
                };

                if !contains_double_quote {
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
                } else {
                    value.format().fmt(f)?;
                }
                return Ok(());
            }
        }

        write!(f, [value_token.format()])
    }
}
