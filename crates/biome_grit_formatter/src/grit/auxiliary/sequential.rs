use crate::prelude::*;
use biome_grit_syntax::GritSequential;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSequential;
impl FormatNodeRule<GritSequential> for FormatGritSequential {
    fn fmt_fields(&self, node: &GritSequential, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
