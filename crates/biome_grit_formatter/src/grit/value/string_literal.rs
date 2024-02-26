use crate::prelude::*;
use biome_grit_syntax::GritStringLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritStringLiteral;
impl FormatNodeRule<GritStringLiteral> for FormatGritStringLiteral {
    fn fmt_fields(&self, node: &GritStringLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
