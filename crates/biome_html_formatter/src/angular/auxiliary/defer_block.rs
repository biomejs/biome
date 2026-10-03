use crate::prelude::*;
use biome_html_syntax::AngularDeferBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferBlock;
impl FormatNodeRule<AngularDeferBlock> for FormatAngularDeferBlock {
    fn fmt_fields(&self, node: &AngularDeferBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
