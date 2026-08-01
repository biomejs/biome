use crate::prelude::*;
use biome_html_syntax::AngularForOpeningBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForOpeningBlock;
impl FormatNodeRule<AngularForOpeningBlock> for FormatAngularForOpeningBlock {
    fn fmt_fields(&self, node: &AngularForOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
