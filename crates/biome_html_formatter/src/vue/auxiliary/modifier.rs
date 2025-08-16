use crate::prelude::*;
use biome_html_syntax::VueModifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueModifier;
impl FormatNodeRule<VueModifier> for FormatVueModifier {
    fn fmt_fields(&self, node: &VueModifier, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
