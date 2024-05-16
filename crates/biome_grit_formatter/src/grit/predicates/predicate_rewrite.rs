use crate::prelude::*;
use biome_grit_syntax::GritPredicateRewrite;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateRewrite;
impl FormatNodeRule<GritPredicateRewrite> for FormatGritPredicateRewrite {
    fn fmt_fields(&self, node: &GritPredicateRewrite, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
