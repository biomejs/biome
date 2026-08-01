use crate::prelude::*;
use biome_html_syntax::AngularDeferOpeningBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferOpeningBlock;
impl FormatNodeRule<AngularDeferOpeningBlock> for FormatAngularDeferOpeningBlock {
    fn fmt_fields(
        &self,
        node: &AngularDeferOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
