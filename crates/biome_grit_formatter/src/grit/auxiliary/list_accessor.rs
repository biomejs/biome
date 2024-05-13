use crate::prelude::*;
use biome_grit_syntax::GritListAccessor;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListAccessor;
impl FormatNodeRule<GritListAccessor> for FormatGritListAccessor {
    fn fmt_fields(&self, node: &GritListAccessor, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
