use crate::prelude::*;
use biome_grit_syntax::GritUnderscore;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritUnderscore;
impl FormatNodeRule<GritUnderscore> for FormatGritUnderscore {
    fn fmt_fields(&self, node: &GritUnderscore, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
