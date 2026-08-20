use crate::prelude::*;
use biome_html_syntax::AngularIfBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularIfBlock;
impl FormatNodeRule<AngularIfBlock> for FormatAngularIfBlock {
    fn fmt_fields(&self, node: &AngularIfBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
