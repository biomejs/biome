use crate::prelude::*;
use biome_html_syntax::HtmlAstroFragment;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroFragment;
impl FormatNodeRule<HtmlAstroFragment> for FormatHtmlAstroFragment {
    fn fmt_fields(&self, node: &HtmlAstroFragment, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
