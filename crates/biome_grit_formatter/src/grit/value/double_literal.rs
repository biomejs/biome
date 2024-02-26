use crate::prelude::*;
use biome_grit_syntax::GritDoubleLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDoubleLiteral;
impl FormatNodeRule<GritDoubleLiteral> for FormatGritDoubleLiteral {
    fn fmt_fields(&self, node: &GritDoubleLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
