use crate::prelude::*;
use biome_grit_syntax::GritPatternIncludes;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternIncludes;
impl FormatNodeRule<GritPatternIncludes> for FormatGritPatternIncludes {
    fn fmt_fields(&self, node: &GritPatternIncludes, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
