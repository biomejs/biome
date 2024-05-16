use crate::prelude::*;
use biome_grit_syntax::GritVariable;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVariable;
impl FormatNodeRule<GritVariable> for FormatGritVariable {
    fn fmt_fields(&self, node: &GritVariable, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
