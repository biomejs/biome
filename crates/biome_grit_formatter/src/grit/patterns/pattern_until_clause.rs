use crate::prelude::*;
use biome_grit_syntax::GritPatternUntilClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternUntilClause;
impl FormatNodeRule<GritPatternUntilClause> for FormatGritPatternUntilClause {
    fn fmt_fields(&self, node: &GritPatternUntilClause, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
