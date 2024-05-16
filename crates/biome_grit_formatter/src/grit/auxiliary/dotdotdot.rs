use crate::prelude::*;
use biome_grit_syntax::GritDotdotdot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDotdotdot;
impl FormatNodeRule<GritDotdotdot> for FormatGritDotdotdot {
    fn fmt_fields(&self, node: &GritDotdotdot, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
