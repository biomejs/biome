use crate::prelude::*;
use biome_grit_syntax::GritRoot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRoot;
impl FormatNodeRule<GritRoot> for FormatGritRoot {
    fn fmt_fields(&self, node: &GritRoot, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
