use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, format_args, write};
use biome_html_syntax::{HtmlString, HtmlStringFields};
use biome_rowan::TextRange;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlString {
    compact: bool,
}

impl FormatNodeRule<HtmlString> for FormatHtmlString {
    fn embedded_node_range(&self, node: &HtmlString, f: &mut HtmlFormatter) -> Option<TextRange> {
        if self.compact || !f.context().should_delegate_fmt_embedded_nodes() {
            return None;
        }

        // we ask the context if the service flagged this node as having embedded content, and if so, we return the range of the embedded content.
        let value_token = node.value_token().ok()?;
        let content = node.inner_string_text().ok()?;
        let range = content.source_range(value_token.text_range());
        f.context().has_embedded_node(range).then_some(range)
    }

    fn fmt_embedded_node(
        &self,
        node: &HtmlString,
        embedded: &dyn Format<HtmlFormatContext>,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let value_token = node.value_token()?;
        let content = node.inner_string_text()?;
        let range = content.source_range(value_token.text_range());
        let token_range = value_token.text_trimmed_range();

        if token_range == range {
            write!(
                f,
                [
                    token("\""),
                    group(&soft_block_indent(&embedded)),
                    token("\"")
                ]
            )
        } else {
            write!(
                f,
                [
                    located_token_text(
                        &value_token,
                        TextRange::new(token_range.start(), range.start())
                    ),
                    group(&soft_block_indent(&embedded)),
                    located_token_text(
                        &value_token,
                        TextRange::new(range.end(), token_range.end())
                    )
                ]
            )
        }
    }

    fn fmt_fields(&self, node: &HtmlString, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlStringFields { value_token } = node.as_fields();

        if self.compact {
            let value_token = value_token.clone()?;
            return format_removed(&value_token).fmt(f);
        }

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
                                token("\""),
                                located_token_text(value, range),
                                token("\""),
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

impl FormatRuleWithOptions<HtmlString> for FormatHtmlString {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
