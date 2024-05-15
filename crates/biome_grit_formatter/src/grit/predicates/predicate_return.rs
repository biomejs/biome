use crate::prelude::*;
use biome_grit_syntax::GritPredicateReturn;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateReturn;
impl FormatNodeRule<GritPredicateReturn> for FormatGritPredicateReturn {
    fn fmt_fields(&self, node: &GritPredicateReturn, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
