use crate::prelude::*;
use biome_html_syntax::AngularSwitchOpeningBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularSwitchOpeningBlock;
impl FormatNodeRule<AngularSwitchOpeningBlock> for FormatAngularSwitchOpeningBlock {
    fn fmt_fields(
        &self,
        node: &AngularSwitchOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
