use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::GlimmerPathSegmentList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPathSegmentList;

impl FormatRule<GlimmerPathSegmentList> for FormatGlimmerPathSegmentList {
    type Context = HtmlFormatContext;

    fn fmt(&self, node: &GlimmerPathSegmentList, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
