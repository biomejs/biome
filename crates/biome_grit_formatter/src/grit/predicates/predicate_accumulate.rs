use crate::prelude::*;
use biome_grit_syntax::GritPredicateAccumulate;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAccumulate;
impl FormatNodeRule<GritPredicateAccumulate> for FormatGritPredicateAccumulate {
    fn fmt_fields(
        &self,
        node: &GritPredicateAccumulate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
