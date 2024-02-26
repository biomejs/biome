use crate::prelude::*;
use biome_grit_syntax::GritPatternContains;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternContains;
impl FormatNodeRule<GritPatternContains> for FormatGritPatternContains {
    fn fmt_fields(&self, node: &GritPatternContains, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
