use crate::prelude::*;
use biome_grit_syntax::GritPredicateNotEqual;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateNotEqual;
impl FormatNodeRule<GritPredicateNotEqual> for FormatGritPredicateNotEqual {
    fn fmt_fields(&self, node: &GritPredicateNotEqual, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
