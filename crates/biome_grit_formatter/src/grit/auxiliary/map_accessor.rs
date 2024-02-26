use crate::prelude::*;
use biome_grit_syntax::GritMapAccessor;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapAccessor;
impl FormatNodeRule<GritMapAccessor> for FormatGritMapAccessor {
    fn fmt_fields(&self, node: &GritMapAccessor, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
