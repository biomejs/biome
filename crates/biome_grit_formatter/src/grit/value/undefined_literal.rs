use crate::prelude::*;
use biome_grit_syntax::GritUndefinedLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritUndefinedLiteral;
impl FormatNodeRule<GritUndefinedLiteral> for FormatGritUndefinedLiteral {
    fn fmt_fields(&self, node: &GritUndefinedLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
