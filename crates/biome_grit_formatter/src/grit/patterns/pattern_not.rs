use crate::prelude::*;
use biome_grit_syntax::GritPatternNot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternNot;
impl FormatNodeRule<GritPatternNot> for FormatGritPatternNot {
    fn fmt_fields(&self, node: &GritPatternNot, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
