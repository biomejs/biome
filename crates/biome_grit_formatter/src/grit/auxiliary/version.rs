use crate::prelude::*;
use biome_grit_syntax::GritVersion;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVersion;
impl FormatNodeRule<GritVersion> for FormatGritVersion {
    fn fmt_fields(&self, node: &GritVersion, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
