use crate::prelude::*;
use biome_grit_syntax::GritPatternLimit;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternLimit;
impl FormatNodeRule<GritPatternLimit> for FormatGritPatternLimit {
    fn fmt_fields(&self, node: &GritPatternLimit, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
