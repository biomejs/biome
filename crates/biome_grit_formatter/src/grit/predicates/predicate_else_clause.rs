use crate::prelude::*;
use biome_grit_syntax::GritPredicateElseClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateElseClause;
impl FormatNodeRule<GritPredicateElseClause> for FormatGritPredicateElseClause {
    fn fmt_fields(
        &self,
        node: &GritPredicateElseClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
