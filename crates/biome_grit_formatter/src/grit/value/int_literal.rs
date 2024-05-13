use crate::prelude::*;
use biome_grit_syntax::GritIntLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritIntLiteral;
impl FormatNodeRule<GritIntLiteral> for FormatGritIntLiteral {
    fn fmt_fields(&self, node: &GritIntLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
