use crate::prelude::*;
use biome_grit_syntax::GritPatternMaybe;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternMaybe;
impl FormatNodeRule<GritPatternMaybe> for FormatGritPatternMaybe {
    fn fmt_fields(&self, node: &GritPatternMaybe, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
