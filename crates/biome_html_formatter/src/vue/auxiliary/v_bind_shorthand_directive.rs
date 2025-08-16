use crate::prelude::*;
use biome_html_syntax::VueVBindShorthandDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVBindShorthandDirective;
impl FormatNodeRule<VueVBindShorthandDirective> for FormatVueVBindShorthandDirective {
    fn fmt_fields(
        &self,
        node: &VueVBindShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
