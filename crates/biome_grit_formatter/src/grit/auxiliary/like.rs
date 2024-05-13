use crate::prelude::*;
use biome_grit_syntax::GritLike;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLike;
impl FormatNodeRule<GritLike> for FormatGritLike {
    fn fmt_fields(&self, node: &GritLike, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
