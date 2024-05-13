use crate::prelude::*;
use biome_grit_syntax::GritMap;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMap;
impl FormatNodeRule<GritMap> for FormatGritMap {
    fn fmt_fields(&self, node: &GritMap, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
