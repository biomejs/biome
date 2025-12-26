use crate::prelude::*;
use biome_html_syntax::VueVForTupleElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForTupleElement;
impl FormatNodeRule<VueVForTupleElement> for FormatVueVForTupleElement {
    fn fmt_fields(&self, node: &VueVForTupleElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
