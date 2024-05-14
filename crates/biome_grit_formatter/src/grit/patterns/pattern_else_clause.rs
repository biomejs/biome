use crate::prelude::*;
use biome_grit_syntax::GritPatternElseClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternElseClause;
impl FormatNodeRule<GritPatternElseClause> for FormatGritPatternElseClause {
    fn fmt_fields(&self, node: &GritPatternElseClause, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
