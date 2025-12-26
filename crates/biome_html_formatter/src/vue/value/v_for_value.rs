use crate::prelude::*;
use biome_html_syntax::VueVForValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForValue;
impl FormatNodeRule<VueVForValue> for FormatVueVForValue {
    fn fmt_fields(&self, node: &VueVForValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
