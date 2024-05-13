use crate::prelude::*;
use biome_grit_syntax::GritPredicateIfElse;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateIfElse;
impl FormatNodeRule<GritPredicateIfElse> for FormatGritPredicateIfElse {
    fn fmt_fields(&self, node: &GritPredicateIfElse, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
