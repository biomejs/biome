use crate::prelude::*;
use biome_html_syntax::HtmlAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeList;
impl FormatRule<HtmlAttributeList> for FormatHtmlAttributeList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlAttributeList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
