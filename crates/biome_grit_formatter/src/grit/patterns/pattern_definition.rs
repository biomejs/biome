use crate::prelude::*;
use biome_grit_syntax::GritPatternDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternDefinition;
impl FormatNodeRule<GritPatternDefinition> for FormatGritPatternDefinition {
    fn fmt_fields(&self, node: &GritPatternDefinition, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
