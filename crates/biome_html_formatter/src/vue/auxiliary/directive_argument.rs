use crate::prelude::*;
use biome_html_syntax::VueDirectiveArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDirectiveArgument;
impl FormatNodeRule<VueDirectiveArgument> for FormatVueDirectiveArgument {
    fn fmt_fields(&self, node: &VueDirectiveArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
