use crate::prelude::*;
use biome_grit_syntax::GritPredicateLess;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateLess;
impl FormatNodeRule<GritPredicateLess> for FormatGritPredicateLess {
    fn fmt_fields(&self, node: &GritPredicateLess, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
