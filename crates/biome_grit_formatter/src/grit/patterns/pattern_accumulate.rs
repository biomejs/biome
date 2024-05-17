use crate::prelude::*;
use biome_grit_syntax::GritPatternAccumulate;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAccumulate;
impl FormatNodeRule<GritPatternAccumulate> for FormatGritPatternAccumulate {
    fn fmt_fields(&self, node: &GritPatternAccumulate, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
