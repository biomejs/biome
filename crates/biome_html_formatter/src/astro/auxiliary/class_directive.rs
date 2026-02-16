use crate::prelude::*;
use biome_html_syntax::AstroClassDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroClassDirective;
impl FormatNodeRule<AstroClassDirective> for FormatAstroClassDirective {
    fn fmt_fields(&self, node: &AstroClassDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
