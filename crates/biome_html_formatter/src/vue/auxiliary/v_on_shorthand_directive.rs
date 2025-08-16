use crate::prelude::*;
use biome_html_syntax::VueVOnShorthandDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVOnShorthandDirective;
impl FormatNodeRule<VueVOnShorthandDirective> for FormatVueVOnShorthandDirective {
    fn fmt_fields(
        &self,
        node: &VueVOnShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
