use crate::prelude::*;
use biome_html_syntax::HtmlRoot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlRoot;
impl FormatNodeRule<HtmlRoot> for FormatHtmlRoot {
    fn fmt_fields(&self, node: &HtmlRoot, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
