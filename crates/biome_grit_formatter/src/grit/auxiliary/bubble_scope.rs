use crate::prelude::*;
use biome_grit_syntax::GritBubbleScope;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBubbleScope;
impl FormatNodeRule<GritBubbleScope> for FormatGritBubbleScope {
    fn fmt_fields(&self, node: &GritBubbleScope, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
