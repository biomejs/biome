use crate::prelude::*;
use biome_html_syntax::HtmlName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlName;
impl FormatNodeRule<HtmlName> for FormatHtmlName {
    fn fmt_fields(&self, node: &HtmlName, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
