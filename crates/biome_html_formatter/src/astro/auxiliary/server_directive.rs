use crate::prelude::*;
use biome_html_syntax::AstroServerDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroServerDirective;
impl FormatNodeRule<AstroServerDirective> for FormatAstroServerDirective {
    fn fmt_fields(&self, node: &AstroServerDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
