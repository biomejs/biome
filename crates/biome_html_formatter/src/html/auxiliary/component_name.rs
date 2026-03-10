use crate::prelude::*;
use biome_html_syntax::{HtmlComponentName, HtmlComponentNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlComponentName;
impl FormatNodeRule<HtmlComponentName> for FormatHtmlComponentName {
    fn fmt_fields(&self, node: &HtmlComponentName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlComponentNameFields { value_token } = node.as_fields();

        value_token.format().fmt(f)
    }
}
