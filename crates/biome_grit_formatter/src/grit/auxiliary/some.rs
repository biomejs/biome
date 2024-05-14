use crate::prelude::*;
use biome_grit_syntax::GritSome;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSome;
impl FormatNodeRule<GritSome> for FormatGritSome {
    fn fmt_fields(&self, node: &GritSome, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
