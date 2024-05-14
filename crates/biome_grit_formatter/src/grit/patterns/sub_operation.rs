use crate::prelude::*;
use biome_grit_syntax::GritSubOperation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSubOperation;
impl FormatNodeRule<GritSubOperation> for FormatGritSubOperation {
    fn fmt_fields(&self, node: &GritSubOperation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
