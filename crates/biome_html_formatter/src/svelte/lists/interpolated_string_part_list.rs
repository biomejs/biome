use crate::prelude::*;
use biome_html_syntax::SvelteInterpolatedStringPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteInterpolatedStringPartList;
impl FormatRule<SvelteInterpolatedStringPartList> for FormatSvelteInterpolatedStringPartList {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &SvelteInterpolatedStringPartList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
