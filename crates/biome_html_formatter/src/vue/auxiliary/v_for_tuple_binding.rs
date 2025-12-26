use crate::prelude::*;
use biome_html_syntax::VueVForTupleBinding;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForTupleBinding;
impl FormatNodeRule<VueVForTupleBinding> for FormatVueVForTupleBinding {
    fn fmt_fields(&self, node: &VueVForTupleBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
