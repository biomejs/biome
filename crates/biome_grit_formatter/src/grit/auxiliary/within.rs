use crate::prelude::*;
use biome_grit_syntax::GritWithin;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritWithin;
impl FormatNodeRule<GritWithin> for FormatGritWithin {
    fn fmt_fields(&self, node: &GritWithin, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
