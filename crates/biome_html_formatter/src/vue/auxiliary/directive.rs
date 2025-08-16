use crate::prelude::*;
use biome_html_syntax::VueDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDirective;
impl FormatNodeRule<VueDirective> for FormatVueDirective {
    fn fmt_fields(&self, node: &VueDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
