use crate::prelude::*;
use biome_grit_syntax::GritPredicateNot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateNot;
impl FormatNodeRule<GritPredicateNot> for FormatGritPredicateNot {
    fn fmt_fields(&self, node: &GritPredicateNot, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
