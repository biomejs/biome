use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlAttributeName, HtmlAttributeNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeName;
impl FormatNodeRule<HtmlAttributeName> for FormatHtmlAttributeName {
    fn fmt_fields(&self, node: &HtmlAttributeName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
