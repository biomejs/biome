use crate::prelude::*;
use biome_grit_syntax::GritMulOperation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMulOperation;
impl FormatNodeRule<GritMulOperation> for FormatGritMulOperation {
    fn fmt_fields(&self, node: &GritMulOperation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
