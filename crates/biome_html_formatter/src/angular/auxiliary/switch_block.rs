use crate::prelude::*;
use biome_html_syntax::AngularSwitchBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularSwitchBlock;
impl FormatNodeRule<AngularSwitchBlock> for FormatAngularSwitchBlock {
    fn fmt_fields(&self, node: &AngularSwitchBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
