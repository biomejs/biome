use crate::prelude::*;
use biome_html_syntax::AstroIsDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroIsDirective;
impl FormatNodeRule<AstroIsDirective> for FormatAstroIsDirective {
    fn fmt_fields(&self, node: &AstroIsDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
