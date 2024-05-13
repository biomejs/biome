use crate::prelude::*;
use biome_grit_syntax::GritPatternAfter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAfter;
impl FormatNodeRule<GritPatternAfter> for FormatGritPatternAfter {
    fn fmt_fields(&self, node: &GritPatternAfter, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
