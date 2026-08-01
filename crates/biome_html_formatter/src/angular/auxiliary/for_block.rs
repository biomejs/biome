use crate::prelude::*;
use biome_html_syntax::AngularForBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForBlock;
impl FormatNodeRule<AngularForBlock> for FormatAngularForBlock {
    fn fmt_fields(&self, node: &AngularForBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
