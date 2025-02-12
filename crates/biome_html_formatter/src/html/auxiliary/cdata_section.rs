use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlCdataSection, HtmlCdataSectionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlCdataSection;
impl FormatNodeRule<HtmlCdataSection> for FormatHtmlCdataSection {
    fn fmt_fields(&self, node: &HtmlCdataSection, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlCdataSectionFields {
            cdata_start_token,
            content_token,
            cdata_end_token,
        } = node.as_fields();

        write!(
            f,
            [
                cdata_start_token.format(),
                content_token.format(),
                cdata_end_token.format()
            ]
        )
    }
}
