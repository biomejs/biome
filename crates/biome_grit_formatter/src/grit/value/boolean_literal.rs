use crate::prelude::*;
use biome_grit_syntax::GritBooleanLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBooleanLiteral;
impl FormatNodeRule<GritBooleanLiteral> for FormatGritBooleanLiteral {
    fn fmt_fields(&self, node: &GritBooleanLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
