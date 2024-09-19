use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlName, HtmlNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlName;
impl FormatNodeRule<HtmlName> for FormatHtmlName {
    fn fmt_fields(&self, node: &HtmlName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
