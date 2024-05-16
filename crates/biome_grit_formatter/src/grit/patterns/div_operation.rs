use crate::prelude::*;
use biome_grit_syntax::GritDivOperation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDivOperation;
impl FormatNodeRule<GritDivOperation> for FormatGritDivOperation {
    fn fmt_fields(&self, node: &GritDivOperation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
