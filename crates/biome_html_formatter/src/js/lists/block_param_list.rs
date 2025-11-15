use crate::prelude::*;
use biome_html_syntax::GlimmerBlockParamList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockParamList;
impl FormatRule<GlimmerBlockParamList> for FormatGlimmerBlockParamList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &GlimmerBlockParamList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
