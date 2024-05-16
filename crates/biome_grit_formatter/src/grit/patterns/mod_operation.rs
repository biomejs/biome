use crate::prelude::*;
use biome_grit_syntax::GritModOperation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritModOperation;
impl FormatNodeRule<GritModOperation> for FormatGritModOperation {
    fn fmt_fields(&self, node: &GritModOperation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
