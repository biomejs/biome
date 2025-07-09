use crate::prelude::*;
use biome_html_syntax::HtmlAstroFragmentClose;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroFragmentClose;
impl FormatNodeRule<HtmlAstroFragmentClose> for FormatHtmlAstroFragmentClose {
    fn fmt_fields(&self, node: &HtmlAstroFragmentClose, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
