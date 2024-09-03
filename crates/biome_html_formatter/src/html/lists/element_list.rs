use crate::prelude::*;
use biome_html_syntax::HtmlElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElementList;
impl FormatRule<HtmlElementList> for FormatHtmlElementList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlElementList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
