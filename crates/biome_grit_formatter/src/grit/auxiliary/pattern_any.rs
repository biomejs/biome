use crate::prelude::*;
use biome_grit_syntax::GritPatternAny;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAny;
impl FormatNodeRule<GritPatternAny> for FormatGritPatternAny {
    fn fmt_fields(&self, node: &GritPatternAny, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
