use crate::prelude::*;
use biome_grit_syntax::GritPredicateAssignment;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAssignment;
impl FormatNodeRule<GritPredicateAssignment> for FormatGritPredicateAssignment {
    fn fmt_fields(
        &self,
        node: &GritPredicateAssignment,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
