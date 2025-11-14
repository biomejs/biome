use crate::prelude::*;
use biome_html_syntax::GlimmerPathSegmentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPathSegmentList;
impl FormatRule<GlimmerPathSegmentList> for FormatGlimmerPathSegmentList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &GlimmerPathSegmentList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
