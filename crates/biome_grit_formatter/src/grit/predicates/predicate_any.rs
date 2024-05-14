use crate::prelude::*;
use biome_grit_syntax::GritPredicateAny;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAny;
impl FormatNodeRule<GritPredicateAny> for FormatGritPredicateAny {
    fn fmt_fields(&self, node: &GritPredicateAny, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
