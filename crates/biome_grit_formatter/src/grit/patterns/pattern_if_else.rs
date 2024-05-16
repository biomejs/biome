use crate::prelude::*;
use biome_grit_syntax::GritPatternIfElse;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternIfElse;
impl FormatNodeRule<GritPatternIfElse> for FormatGritPatternIfElse {
    fn fmt_fields(&self, node: &GritPatternIfElse, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
