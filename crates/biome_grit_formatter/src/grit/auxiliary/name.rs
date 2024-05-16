use crate::prelude::*;
use biome_grit_syntax::GritName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritName;
impl FormatNodeRule<GritName> for FormatGritName {
    fn fmt_fields(&self, node: &GritName, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
