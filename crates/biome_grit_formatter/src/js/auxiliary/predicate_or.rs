use crate::prelude::*;
use biome_grit_syntax::GritPredicateOr;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateOr;
impl FormatNodeRule<GritPredicateOr> for FormatGritPredicateOr {
    fn fmt_fields(&self, node: &GritPredicateOr, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
