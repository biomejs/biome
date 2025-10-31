use crate::prelude::*;
use biome_html_syntax::GlimmerArgumentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerArgumentList;
impl FormatRule<GlimmerArgumentList> for FormatGlimmerArgumentList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &GlimmerArgumentList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
