use crate::prelude::*;
use biome_grit_syntax::GritFunctionDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritFunctionDefinition;
impl FormatNodeRule<GritFunctionDefinition> for FormatGritFunctionDefinition {
    fn fmt_fields(&self, node: &GritFunctionDefinition, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
