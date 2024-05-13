use crate::prelude::*;
use biome_grit_syntax::GritBubble;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBubble;
impl FormatNodeRule<GritBubble> for FormatGritBubble {
    fn fmt_fields(&self, node: &GritBubble, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
