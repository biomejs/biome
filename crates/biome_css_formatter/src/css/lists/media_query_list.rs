use crate::prelude::*;
use crate::utils::media_query_comments::fill_media_queries;
use biome_css_syntax::CssMediaQueryList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryList;
impl FormatRule<CssMediaQueryList> for FormatCssMediaQueryList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssMediaQueryList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut fill = f.fill();
        fill_media_queries(node, |separator, formatted| {
            fill.entry(separator, formatted);
        });
        fill.finish()
    }
}
