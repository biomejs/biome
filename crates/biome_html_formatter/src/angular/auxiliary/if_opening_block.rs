use crate::prelude::*;
use biome_html_syntax::AngularIfOpeningBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularIfOpeningBlock;
impl FormatNodeRule<AngularIfOpeningBlock> for FormatAngularIfOpeningBlock {
    fn fmt_fields(&self, node: &AngularIfOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
