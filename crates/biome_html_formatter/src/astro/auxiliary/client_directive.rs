use crate::prelude::*;
use biome_html_syntax::AstroClientDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroClientDirective;
impl FormatNodeRule<AstroClientDirective> for FormatAstroClientDirective {
    fn fmt_fields(&self, node: &AstroClientDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
