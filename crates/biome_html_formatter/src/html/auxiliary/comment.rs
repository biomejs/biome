use crate::prelude::*;
use biome_html_syntax::HtmlComment;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlComment;
impl FormatNodeRule<HtmlComment> for FormatHtmlComment {
    fn fmt_fields(&self, node: &HtmlComment, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
