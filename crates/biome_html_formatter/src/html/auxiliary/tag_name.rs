use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlTagName, HtmlTagNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTagName;
impl FormatNodeRule<HtmlTagName> for FormatHtmlTagName {
    fn fmt_fields(&self, node: &HtmlTagName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlTagNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
