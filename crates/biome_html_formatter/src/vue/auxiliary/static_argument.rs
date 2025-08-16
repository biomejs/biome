use crate::prelude::*;
use biome_html_syntax::VueStaticArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueStaticArgument;
impl FormatNodeRule<VueStaticArgument> for FormatVueStaticArgument {
    fn fmt_fields(&self, node: &VueStaticArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
