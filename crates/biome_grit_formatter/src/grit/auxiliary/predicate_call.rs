use crate::prelude::*;
use biome_grit_syntax::GritPredicateCall;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateCall;
impl FormatNodeRule<GritPredicateCall> for FormatGritPredicateCall {
    fn fmt_fields(&self, node: &GritPredicateCall, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
