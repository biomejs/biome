use crate::prelude::*;
use biome_grit_syntax::GritPatternBefore;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternBefore;
impl FormatNodeRule<GritPatternBefore> for FormatGritPatternBefore {
    fn fmt_fields(&self, node: &GritPatternBefore, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
