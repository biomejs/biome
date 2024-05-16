use crate::prelude::*;
use biome_grit_syntax::GritPredicateGreaterEqual;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateGreaterEqual;
impl FormatNodeRule<GritPredicateGreaterEqual> for FormatGritPredicateGreaterEqual {
    fn fmt_fields(
        &self,
        node: &GritPredicateGreaterEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
