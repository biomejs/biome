use crate::prelude::*;
use biome_grit_syntax::GritPatternAnd;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAnd;
impl FormatNodeRule<GritPatternAnd> for FormatGritPatternAnd {
    fn fmt_fields(&self, node: &GritPatternAnd, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
