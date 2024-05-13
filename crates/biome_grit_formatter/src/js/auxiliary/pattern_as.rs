use crate::prelude::*;
use biome_grit_syntax::GritPatternAs;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAs;
impl FormatNodeRule<GritPatternAs> for FormatGritPatternAs {
    fn fmt_fields(&self, node: &GritPatternAs, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
