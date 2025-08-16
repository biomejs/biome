use crate::prelude::*;
use biome_html_syntax::VueDynamicArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDynamicArgument;
impl FormatNodeRule<VueDynamicArgument> for FormatVueDynamicArgument {
    fn fmt_fields(&self, node: &VueDynamicArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
