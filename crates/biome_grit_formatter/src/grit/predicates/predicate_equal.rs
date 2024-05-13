use crate::prelude::*;
use biome_grit_syntax::GritPredicateEqual;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateEqual;
impl FormatNodeRule<GritPredicateEqual> for FormatGritPredicateEqual {
    fn fmt_fields(&self, node: &GritPredicateEqual, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
