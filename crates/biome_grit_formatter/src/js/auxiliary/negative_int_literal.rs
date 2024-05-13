use crate::prelude::*;
use biome_grit_syntax::GritNegativeIntLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNegativeIntLiteral;
impl FormatNodeRule<GritNegativeIntLiteral> for FormatGritNegativeIntLiteral {
    fn fmt_fields(&self, node: &GritNegativeIntLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
