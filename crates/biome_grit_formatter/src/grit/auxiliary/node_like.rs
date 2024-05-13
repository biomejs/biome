use crate::prelude::*;
use biome_grit_syntax::GritNodeLike;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNodeLike;
impl FormatNodeRule<GritNodeLike> for FormatGritNodeLike {
    fn fmt_fields(&self, node: &GritNodeLike, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
