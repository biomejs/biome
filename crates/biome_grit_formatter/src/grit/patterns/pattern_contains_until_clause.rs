use crate::prelude::*;
use biome_grit_syntax::GritPatternContainsUntilClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternContainsUntilClause;
impl FormatNodeRule<GritPatternContainsUntilClause> for FormatGritPatternContainsUntilClause {
    fn fmt_fields(
        &self,
        node: &GritPatternContainsUntilClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
