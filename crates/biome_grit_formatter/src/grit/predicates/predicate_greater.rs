use crate::prelude::*;
use biome_grit_syntax::GritPredicateGreater;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateGreater;
impl FormatNodeRule<GritPredicateGreater> for FormatGritPredicateGreater {
    fn fmt_fields(&self, node: &GritPredicateGreater, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
