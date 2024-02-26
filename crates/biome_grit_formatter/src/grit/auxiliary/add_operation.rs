use crate::prelude::*;
use biome_grit_syntax::GritAddOperation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAddOperation;
impl FormatNodeRule<GritAddOperation> for FormatGritAddOperation {
    fn fmt_fields(&self, node: &GritAddOperation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
