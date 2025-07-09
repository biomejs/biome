use crate::prelude::*;
use biome_html_syntax::HtmlAstroFragmentOpen;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroFragmentOpen;
impl FormatNodeRule<HtmlAstroFragmentOpen> for FormatHtmlAstroFragmentOpen {
    fn fmt_fields(&self, node: &HtmlAstroFragmentOpen, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
