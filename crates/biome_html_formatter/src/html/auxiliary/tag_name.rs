use crate::{
    prelude::*,
    utils::{formatters::FormatTokenAsLowercase, metadata::should_lowercase_html_tag},
};
use biome_formatter::write;
use biome_html_syntax::{HtmlTagName, HtmlTagNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTagName;
impl FormatNodeRule<HtmlTagName> for FormatHtmlTagName {
    fn fmt_fields(&self, node: &HtmlTagName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlTagNameFields { value_token } = node.as_fields();

        if should_lowercase_html_tag(f, node) {
            write!(f, [FormatTokenAsLowercase::from(value_token?)])
        } else {
            write![f, [value_token.format()]]
        }
    }
}
