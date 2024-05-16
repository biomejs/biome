use crate::prelude::*;
use biome_grit_syntax::GritPredicateAnd;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAnd;
impl FormatNodeRule<GritPredicateAnd> for FormatGritPredicateAnd {
    fn fmt_fields(&self, node: &GritPredicateAnd, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
