use crate::prelude::*;
use biome_grit_syntax::GritEvery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritEvery;
impl FormatNodeRule<GritEvery> for FormatGritEvery {
    fn fmt_fields(&self, node: &GritEvery, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
