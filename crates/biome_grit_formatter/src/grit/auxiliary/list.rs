use crate::prelude::*;
use biome_grit_syntax::GritList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritList;
impl FormatNodeRule<GritList> for FormatGritList {
    fn fmt_fields(&self, node: &GritList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
