use crate::prelude::*;
use biome_grit_syntax::GritLikeThreshold;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLikeThreshold;
impl FormatNodeRule<GritLikeThreshold> for FormatGritLikeThreshold {
    fn fmt_fields(&self, node: &GritLikeThreshold, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
