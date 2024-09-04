use crate::prelude::*;
use biome_html_syntax::HtmlDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDirective;
impl FormatNodeRule<HtmlDirective> for FormatHtmlDirective {
    fn fmt_fields(&self, node: &HtmlDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
