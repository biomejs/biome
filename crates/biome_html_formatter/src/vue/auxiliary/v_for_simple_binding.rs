use crate::prelude::*;
use biome_html_syntax::VueVForSimpleBinding;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForSimpleBinding;
impl FormatNodeRule<VueVForSimpleBinding> for FormatVueVForSimpleBinding {
    fn fmt_fields(&self, node: &VueVForSimpleBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
