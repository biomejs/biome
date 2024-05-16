use crate::prelude::*;
use biome_grit_syntax::GritDot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDot;
impl FormatNodeRule<GritDot> for FormatGritDot {
    fn fmt_fields(&self, node: &GritDot, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
