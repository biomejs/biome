use crate::prelude::*;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttribute;
impl FormatNodeRule<HtmlAttribute> for FormatHtmlAttribute {
    fn fmt_fields(&self, node: &HtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
