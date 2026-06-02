use crate::prelude::*;
use biome_html_syntax::SvelteTemplateElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTemplateElementList;
impl FormatRule<SvelteTemplateElementList> for FormatSvelteTemplateElementList {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &SvelteTemplateElementList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
