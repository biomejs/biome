use crate::prelude::*;
use biome_grit_syntax::GritPredicateLessEqual;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateLessEqual;
impl FormatNodeRule<GritPredicateLessEqual> for FormatGritPredicateLessEqual {
    fn fmt_fields(&self, node: &GritPredicateLessEqual, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
